// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{config, empty_chain};
use crate::{
    persistence::{handoff::observe_handoff, worker::install_record_append_pause},
    sync::applied::*,
};
use std::{
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

#[test]
fn two_verified_applications_and_ram_capture_continue_before_first_append_resumes() {
    let directory = tempfile::tempdir().unwrap();
    let chain = empty_chain();
    let path = directory.path().join("applied");
    let (mut owner, reader) = open_applied_state_service(config(&path, &chain), &chain.genesis)
        .ok()
        .unwrap();
    let genesis = capture_applied_state(&reader).unwrap();
    let (entered, resume) = install_record_append_pause(
        crate::sync::applied::admission::compact_worker(&owner).unwrap(),
    );
    let first = try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    let first_view = capture_applied_state(&reader).unwrap();
    let second_bytes = chain.records[1].clone();
    let (sender, receiver) = mpsc::sync_channel(1);
    let apply = thread::spawn(move || {
        let second = try_apply_recovery_bytes(&mut owner, &second_bytes);
        sender.send((owner, second)).ok().unwrap();
    });
    let second = receiver.recv_timeout(Duration::from_secs(1));
    let captured_reader = reader.clone();
    let (captured, capture_result) = mpsc::sync_channel(1);
    let read = thread::spawn(move || {
        captured
            .send(capture_applied_state(&captured_reader))
            .ok()
            .unwrap()
    });
    let view = capture_result.recv_timeout(Duration::from_secs(1));
    // Always release the real writer before assertions, including timeout failures.
    resume.send(()).unwrap();
    apply.join().unwrap();
    read.join().unwrap();
    let (owner, second) = second.expect("second admission must complete before append resumes");
    let second = second.unwrap();
    let view = view
        .expect("RAM capture must complete while append is paused")
        .unwrap();
    assert_eq!(first.applied.0, 1);
    assert_eq!(second.applied.0, 2);
    assert_eq!(applied_markers(&view).applied.0, 2);
    assert_eq!(applied_markers(&view).authenticated_state.0, 2);
    assert_eq!(applied_markers(&view).finalized.0, 3);
    assert_eq!(applied_markers(&view).durable_recovery.0, 0);
    assert_eq!(applied_commit(&genesis), &chain.commits[0]);
    assert_eq!(applied_commit(&first_view), &chain.commits[1]);
    assert_eq!(applied_commit(&view), &chain.commits[2]);
    assert_eq!(applied_anchor(&view).unwrap().execution_height(), 2);
    let retained =
        observe_handoff(crate::sync::applied::admission::compact_pool(&owner).unwrap()).unwrap();
    assert_eq!(retained.retained_batches, 2);
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.acknowledgement_error.is_none());
    assert_eq!(retained_tail_len(&shutdown.unacknowledged_tail), 0);
    assert_eq!(
        applied_markers(shutdown.publication.as_ref().unwrap())
            .durable_recovery
            .0,
        2
    );
    drop(shutdown.repository.unwrap());
    let (reopened, reopened_reader) =
        open_applied_state_service(config(&path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    let restored = capture_applied_state(&reopened_reader).unwrap();
    assert_eq!(applied_commit(&restored), &chain.commits[2]);
    assert_eq!(
        applied_cursors(&restored),
        applied_cursors(shutdown.publication.as_ref().unwrap())
    );
    assert_eq!(applied_markers(&restored).durable_recovery.0, 2);
    drop(finish_applied_state_service(reopened));
    assert!(!Arc::ptr_eq(&genesis, &first_view));
}
