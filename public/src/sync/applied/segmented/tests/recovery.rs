// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{configuration, logical_chain};
use crate::persistence::segmented::{
    bind_segmented_reservation, install_segmented_record_pause, plan_segmented_layout,
    reserve_segmented_layout, segmented_part_bytes, write_and_seal_segmented_batch,
};
use crate::sync::applied::types::AppliedBackend;
use crate::sync::applied::*;
use eve_storage::records::segmented::{
    SegmentedLogicalIdentity, SegmentedRecoveryMode, hash_segmented_logical_body,
};
use eve_storage::records::{
    compare_and_append_opaque_records, opaque_record_cursor, open_opaque_record_repository,
};
use std::{sync::Arc, time::Duration};

#[test]
fn complete_unmarked_data_recovers_only_verified_prefix_then_resumes_after_orphan_without_reset() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("segmented");
    let chain = logical_chain(false);
    let (mut owner, reader) =
        open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    let AppliedBackend::Segmented { worker, .. } = &owner.backend else {
        panic!("segmented");
    };
    let (entered, resume) = install_segmented_record_pause(worker.as_ref().unwrap(), 1, true);
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    entered.recv_timeout(Duration::from_secs(10)).unwrap();
    resume.send(()).unwrap();
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.repository.is_err());
    let progress = retained_tail_progress(&shutdown.unacknowledged_tail, 0).unwrap();
    assert_eq!(progress.last_acknowledged_physical.sequence, 1);
    assert_eq!(progress.logical_target, 1);
    assert!(progress.complete_marker.is_none());
    assert_eq!(
        applied_markers(&capture_applied_state(&reader).unwrap())
            .durable_recovery
            .0,
        0
    );
    let (mut resumed, recovered_reader) =
        open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    let restored = capture_applied_state(&recovered_reader).unwrap();
    assert_eq!(applied_commit(&restored), &chain.commits[0]);
    assert_eq!(
        applied_segmented_position(&restored).unwrap().missing_from,
        Some(1)
    );
    assert_eq!(
        (
            applied_cursors(&restored).0.sequence,
            applied_cursors(&restored).1.sequence
        ),
        (1, 0)
    );
    assert!(matches!(
        applied_readiness(&restored),
        eve_node_policy::PublicReadiness::NotReady(_)
    ));
    try_apply_recovery_bytes(&mut resumed, &chain.records[0]).unwrap();
    let finished = finish_applied_state_service(resumed);
    assert!(finished.acknowledgement_error.is_none());
    let final_view = finished.publication.as_ref().unwrap();
    assert_eq!(
        (
            applied_markers(final_view).durable_recovery.0,
            applied_cursors(final_view).1.sequence
        ),
        (1, 3)
    );
    assert_eq!(
        applied_segmented_position(final_view).unwrap().missing_from,
        None
    );
    drop(finished.repository.unwrap());
    let (reopened, reread) =
        open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    assert_eq!(
        applied_commit(&capture_applied_state(&reread).unwrap()),
        &chain.commits[1]
    );
    drop(finish_applied_state_service(reopened));
}

#[test]
fn physically_complete_integrity_valid_marker_with_invalid_v2_body_never_advances_verified_state() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("segmented");
    let chain = logical_chain(false);
    let (owner, reader) =
        open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    let position = applied_segmented_position(&capture_applied_state(&reader).unwrap()).unwrap();
    let AppliedBackend::Segmented { pool, .. } = &owner.backend else {
        panic!("segmented");
    };
    let pool = Arc::clone(pool);
    let namespace = owner.effective_storage_identity;
    let budget = owner.config.repository_budget;
    let shutdown = finish_applied_state_service(owner);
    let mut repository = shutdown.repository.unwrap();
    let bytes = b"invalid-logical-wire";
    let identity = SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: hash_segmented_logical_body(bytes),
        parent: position.durable,
        target_height: 1,
        total_length: bytes.len() as u64,
    };
    let layout = plan_segmented_layout(&pool, identity).unwrap();
    let reservation = reserve_segmented_layout(&pool, &layout).unwrap();
    let bound = bind_segmented_reservation(reservation, [9; 32])
        .ok()
        .unwrap();
    let batch = write_and_seal_segmented_batch(bound, bytes, position.durable.cursor).unwrap();
    for part in 0..2 {
        let payload = segmented_part_bytes(&batch, part, 0).unwrap().to_vec();
        let parent = opaque_record_cursor(&repository).unwrap();
        compare_and_append_opaque_records(&mut repository, parent, &[payload]).unwrap();
    }
    let stored = opaque_record_cursor(&repository).unwrap();
    drop(repository);
    assert!(matches!(
        open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis),
        Err(AppliedError::ImportWire(_))
    ));
    let repository = open_opaque_record_repository(&path, namespace, budget).unwrap();
    assert_eq!(opaque_record_cursor(&repository).unwrap(), stored);
}
