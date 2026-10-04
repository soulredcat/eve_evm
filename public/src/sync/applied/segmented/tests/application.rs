// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{configuration, logical_chain};
use crate::persistence::segmented::install_segmented_record_pause;
use crate::sync::applied::types::AppliedBackend;
use crate::sync::applied::*;
use std::{sync::mpsc, thread, time::Duration};

#[test]
fn large_v2_import_and_ram_reads_continue_before_first_sync_then_reopen_verified_logical_prefix() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("segmented");
    let chain = logical_chain(true);
    assert!(chain.records[0].len() > 4_198_312);
    let (mut owner, reader) =
        open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    let genesis = capture_applied_state(&reader).unwrap();
    let AppliedBackend::Segmented { worker, .. } = &owner.backend else {
        panic!("segmented backend");
    };
    let (entered, resume) = install_segmented_record_pause(worker.as_ref().unwrap(), 0, false);
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    entered.recv_timeout(Duration::from_secs(10)).unwrap();
    let first = capture_applied_state(&reader).unwrap();
    let second_body = chain.records[1].clone();
    let (sender, receiver) = mpsc::sync_channel(1);
    let applicant = thread::spawn(move || {
        let result = try_apply_recovery_bytes(&mut owner, &second_body);
        sender.send((owner, result)).ok().unwrap();
    });
    let second = receiver.recv_timeout(Duration::from_secs(3));
    let query_reader = reader.clone();
    let (query_sender, query_receiver) = mpsc::sync_channel(1);
    let query = thread::spawn(move || {
        query_sender
            .send(capture_applied_state(&query_reader))
            .ok()
            .unwrap()
    });
    let captured = query_receiver.recv_timeout(Duration::from_secs(1));
    resume.send(()).unwrap();
    applicant.join().unwrap();
    query.join().unwrap();
    let (owner, admitted) =
        second.expect("bounded prepared admission must finish before paused disk resumes");
    assert_eq!(admitted.unwrap().applied.0, 2);
    let latest = captured
        .expect("RAM read is independent of paused disk")
        .unwrap();
    assert_eq!(applied_commit(&first), &chain.commits[1]);
    assert_eq!(applied_commit(&latest), &chain.commits[2]);
    assert_eq!(applied_commit(&genesis), &chain.commits[0]);
    assert_eq!(applied_markers(&latest).durable_recovery.0, 0);
    assert_eq!(
        applied_segmented_position(&latest).unwrap().applied.height,
        2
    );
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.acknowledgement_error.is_none());
    assert_eq!(retained_tail_len(&shutdown.unacknowledged_tail), 0);
    let final_view = shutdown.publication.as_ref().unwrap();
    assert_eq!(applied_markers(final_view).durable_recovery.0, 2);
    assert!(applied_cursors(final_view).0.sequence > 2);
    drop(shutdown.repository.unwrap());
    let (reopened, restored_reader) =
        open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    let restored = capture_applied_state(&restored_reader).unwrap();
    assert_eq!(applied_commit(&restored), &chain.commits[2]);
    assert_eq!(applied_markers(&restored).durable_recovery.0, 2);
    assert!(matches!(
        applied_readiness(&restored),
        eve_node_policy::PublicReadiness::NotReady(_)
    ));
    drop(finish_applied_state_service(reopened));
}
