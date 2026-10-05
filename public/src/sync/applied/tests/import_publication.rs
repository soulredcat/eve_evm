// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::import_fixtures::{import_chain, import_config};
use crate::{persistence::worker::install_record_append_pause, sync::applied::*};
use eve_state::{SystemValue, U256};
use std::{sync::mpsc, thread, time::Duration};

#[test]
fn default_budget_imports_signed_nonempty_state_without_double_fees_during_paused_append_and_reopen()
 {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("import");
    let chain = import_chain();
    let (mut owner, reader) = open_applied_state_service_with_mode(
        import_config(&path, &chain),
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    assert_eq!(
        observe_estimated_working(&reader).unwrap().limit,
        eve_node_policy::development_public_budget().maximum_working_state_bytes
    );
    let genesis = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_mode(&genesis), AppliedMode::AuthenticatedImport);
    assert_eq!(applied_commit(&genesis), &chain.commits[0]);
    let (entered, resume) = install_record_append_pause(owner.worker.as_ref().unwrap());
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    let first = capture_applied_state(&reader).unwrap();
    let record = chain.records[1].clone();
    let (sender, receiver) = mpsc::sync_channel(1);
    let applicant = thread::spawn(move || {
        let result = try_apply_recovery_bytes(&mut owner, &record);
        sender.send((owner, result)).ok().unwrap();
    });
    let result = receiver.recv_timeout(Duration::from_secs(1));
    let query_reader = reader.clone();
    let (captured, capture_receiver) = mpsc::sync_channel(1);
    let query = thread::spawn(move || {
        captured
            .send(capture_applied_state(&query_reader))
            .ok()
            .unwrap()
    });
    let queried = capture_receiver.recv_timeout(Duration::from_secs(1));
    resume.send(()).unwrap();
    applicant.join().unwrap();
    query.join().unwrap();
    let (owner, admitted) = result.expect("second import must admit before storage resumes");
    assert_eq!(admitted.unwrap().applied.0, 2);
    let latest = queried
        .expect("charged RAM capture must not wait for storage")
        .unwrap();
    assert_eq!(applied_markers(&latest).durable_recovery.0, 0);
    assert_eq!(applied_markers(&latest).authenticated_state.0, 2);
    assert_eq!(applied_commit(&first), &chain.commits[1]);
    assert_eq!(applied_commit(&latest), &chain.commits[2]);
    assert_eq!(applied_commit(&genesis), &chain.commits[0]);
    assert!(!applied_commit(&first).block.transactions.is_empty());
    assert!(applied_commit(&first).block.header.gas_used > 0);
    let burned = applied_commit(&first)
        .state
        .system
        .values()
        .find_map(|record| match record.value {
            SystemValue::Fee { burned, .. } => Some(burned),
            _ => None,
        })
        .unwrap();
    assert!(burned > U256::ZERO);
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
    let (reopened, restored_reader) = open_applied_state_service_with_mode(
        import_config(&path, &chain),
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    let restored = capture_applied_state(&restored_reader).unwrap();
    assert_eq!(applied_mode(&restored), AppliedMode::AuthenticatedImport);
    assert_eq!(applied_commit(&restored), &chain.commits[2]);
    assert_eq!(applied_markers(&restored).durable_recovery.0, 2);
    assert_eq!(
        applied_cursors(&restored),
        applied_cursors(shutdown.publication.as_ref().unwrap())
    );
    drop(finish_applied_state_service(reopened));
}
