// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{configuration, logical_chain};
use crate::persistence::segmented::{install_segmented_record_pause, observe_segmented_parts};
use crate::sync::applied::types::AppliedBackend;
use crate::sync::applied::*;
use eve_storage::records::{
    compare_and_append_opaque_records, opaque_record_cursor, open_opaque_record_repository,
    read_opaque_record,
};
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

/// SIMULATED_UNSYNCED_LOSS: copy only acknowledged real WAL rows to a fresh
/// namespace. The original synced records are preserved; no power loss is claimed.
#[test]
fn simulated_unsynced_marker_loss_restores_nonempty_prefix_and_retrieves_authenticated_empty_tail_once()
 {
    let directory = tempfile::tempdir().unwrap();
    let original_path = directory.path().join("original");
    let recovered_path = directory.path().join("simulated-loss");
    let chain = logical_chain(false);
    assert_eq!(chain.inputs.len(), 2);
    assert!(!chain.commits[1].block.transactions.is_empty());
    assert!(!chain.commits[1].block.receipts.is_empty());
    assert!(chain.commits[2].block.transactions.is_empty());
    assert!(chain.records.iter().all(|body| body.len() < 4_194_095));
    let (mut owner, reader) =
        open_segmented_applied_state_service(configuration(&original_path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    let identity = owner.effective_storage_identity;
    let budget = owner.config.repository_budget;
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while poll_applied_durability(&mut owner)
        .unwrap()
        .durable_recovery
        .0
        < 1
    {
        assert!(
            Instant::now() < deadline,
            "actual H1 durable marker deadline"
        );
        thread::sleep(Duration::from_millis(5));
    }
    let first = capture_applied_state(&reader).unwrap();
    let first_position = applied_segmented_position(&first).unwrap();
    assert_eq!(applied_commit(&first), &chain.commits[1]);
    assert_eq!(first_position.durable.cursor.sequence, 2);
    let AppliedBackend::Segmented { worker, pool, .. } = &owner.backend else {
        panic!("segmented backend");
    };
    let pool = Arc::clone(pool);
    // One data record is truly synced; the pause fires before H2's marker append.
    let (entered, resume) = install_segmented_record_pause(worker.as_ref().unwrap(), 1, true);
    try_apply_recovery_bytes(&mut owner, &chain.records[1]).unwrap();
    entered.recv_timeout(Duration::from_secs(10)).unwrap();
    let second = capture_applied_state(&reader).unwrap();
    let held_parts = observe_segmented_parts(&pool).unwrap();
    let charged = observe_estimated_working(&reader).unwrap();
    // Capture remains responsive while the actual isolated writer is paused.
    let query_reader = reader.clone();
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let query = thread::spawn(move || sender.send(capture_applied_state(&query_reader)).unwrap());
    let concurrent = receiver.recv_timeout(Duration::from_secs(1));
    // Resume before assertions so assertion failures cannot leave this worker blocked.
    resume.send(()).unwrap();
    query.join().unwrap();
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.repository.is_err());
    assert!(shutdown.acknowledgement_error.is_some());
    assert_eq!(applied_commit(&first), &chain.commits[1]);
    assert_eq!(applied_commit(&second), &chain.commits[2]);
    assert!(Arc::ptr_eq(&second, &concurrent.unwrap().unwrap()));
    assert_eq!(applied_markers(&second).durable_recovery.0, 1);
    assert_eq!(held_parts.retained_parts, 2);
    assert!(held_parts.retained_encoded_bytes > 0 && held_parts.estimated_metadata_bytes > 0);
    assert!(
        charged.reserved_estimated_bytes > 0 && charged.reserved_estimated_bytes <= charged.limit
    );
    assert_eq!(retained_tail_len(&shutdown.unacknowledged_tail), 1);
    let progress = retained_tail_progress(&shutdown.unacknowledged_tail, 0).unwrap();
    assert_eq!(progress.logical_target, 2);
    assert!(progress.complete_marker.is_none());
    assert_eq!(progress.last_acknowledged_physical.sequence, 3);
    assert!(retained_tail_part_bytes(&shutdown.unacknowledged_tail, 0, 0, 0).is_some());
    assert!(retained_tail_part_bytes(&shutdown.unacknowledged_tail, 0, 1, 0).is_some());
    let source = open_opaque_record_repository(&original_path, identity, budget).unwrap();
    assert_eq!(
        opaque_record_cursor(&source).unwrap(),
        progress.last_acknowledged_physical
    );
    let mut clone = open_opaque_record_repository(&recovered_path, identity, budget).unwrap();
    // Exactly three actual acknowledged rows, including H2 data but no H2 marker.
    for sequence in 1..=progress.last_acknowledged_physical.sequence {
        let record = read_opaque_record(&source, sequence).unwrap().unwrap();
        let parent = opaque_record_cursor(&clone).unwrap();
        compare_and_append_opaque_records(&mut clone, parent, &[record.payload]).unwrap();
    }
    assert_eq!(
        opaque_record_cursor(&clone).unwrap(),
        progress.last_acknowledged_physical
    );
    assert!(read_opaque_record(&source, 4).unwrap().is_none());
    drop((source, clone));
    assert_eq!(observe_segmented_parts(&pool).unwrap().retained_parts, 2);
    let (mut recovered, recovered_reader) = open_segmented_applied_state_service(
        configuration(&recovered_path, &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let restored = capture_applied_state(&recovered_reader).unwrap();
    assert_eq!(applied_commit(&restored), &chain.commits[1]);
    assert_eq!(applied_markers(&restored).durable_recovery.0, 1);
    let position = applied_segmented_position(&restored).unwrap();
    assert_eq!(position.missing_from, Some(2));
    assert_eq!(
        position.last_acknowledged_physical,
        progress.last_acknowledged_physical
    );
    assert!(matches!(
        applied_readiness(&restored),
        eve_node_policy::PublicReadiness::NotReady(_)
    ));
    let mut forged = chain.inputs[1].clone();
    forged.finalized.commit.signatures[0].signature[0] ^= 1;
    let forged =
        eve_finality_verifier::encode_logical_import_wire(&forged, &recovered.config.state_budget)
            .unwrap();
    // Missing bytes, a stale wrong parent and a forged certificate preserve H1.
    for bytes in [&[][..], chain.records[0].as_slice(), forged.as_slice()] {
        assert!(try_apply_recovery_bytes(&mut recovered, bytes).is_err());
        assert!(Arc::ptr_eq(
            &restored,
            &capture_applied_state(&recovered_reader).unwrap()
        ));
        assert_eq!(recovered.admitted_cursor, applied_cursors(&restored).0);
    }
    // The exact original H2 body is reauthenticated against recovered H1/H+1.
    try_apply_recovery_bytes(&mut recovered, &chain.records[1]).unwrap();
    let caught_up = capture_applied_state(&recovered_reader).unwrap();
    assert_eq!(applied_commit(&caught_up), &chain.commits[2]);
    assert_eq!(applied_commit(&restored), &chain.commits[1]);
    assert!(try_apply_recovery_bytes(&mut recovered, &chain.records[1]).is_err());
    assert!(Arc::ptr_eq(
        &caught_up,
        &capture_applied_state(&recovered_reader).unwrap()
    ));
    let finished = finish_applied_state_service(recovered);
    assert!(finished.acknowledgement_error.is_none());
    assert_eq!(retained_tail_len(&finished.unacknowledged_tail), 0);
    let final_view = finished.publication.as_ref().unwrap();
    assert_eq!(applied_commit(final_view), &chain.commits[2]);
    assert_eq!(applied_markers(final_view).durable_recovery.0, 2);
    assert_eq!(
        applied_segmented_position(final_view).unwrap().missing_from,
        None
    );
    assert_eq!(applied_cursors(final_view).1.sequence, 5);
    drop(finished.repository.unwrap());
    let (reopened, reread) = open_segmented_applied_state_service(
        configuration(&recovered_path, &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    assert_eq!(
        applied_commit(&capture_applied_state(&reread).unwrap()),
        &chain.commits[2]
    );
    drop(finish_applied_state_service(reopened));
    // The failed source's complete charged tail survives until explicitly dropped.
    assert_eq!(observe_segmented_parts(&pool).unwrap().retained_parts, 2);
    drop(shutdown);
    assert_eq!(observe_segmented_parts(&pool).unwrap().retained_parts, 0);
}
