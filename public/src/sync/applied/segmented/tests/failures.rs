// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{configuration, logical_chain};
use crate::persistence::segmented::{
    finish_segmented_worker, observe_segmented_parts, plan_segmented_layout,
    reserve_segmented_layout, try_receive_segmented_ack,
};
use crate::sync::applied::types::{AppliedBackend, PendingPayload};
use crate::sync::applied::*;
use eve_storage::records::segmented::{
    SegmentedLogicalIdentity, SegmentedRecoveryMode, hash_segmented_logical_body,
};
use std::sync::Arc;

#[test]
fn failed_v2_preflight_proof_and_all_part_capacity_preserve_the_published_state() {
    let directory = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (mut owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("segmented"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let before = capture_applied_state(&reader).unwrap();
    let mut input = chain.inputs[0].clone();
    input.finalized.commit.signatures[0].signature[0] ^= 1;
    let invalid_proof =
        eve_finality_verifier::encode_logical_import_wire(&input, &owner.config.state_budget)
            .unwrap();
    for bytes in [b"invalid-v2".to_vec(), invalid_proof] {
        assert!(try_apply_recovery_bytes(&mut owner, &bytes).is_err());
        assert!(Arc::ptr_eq(
            &before,
            &capture_applied_state(&reader).unwrap()
        ));
        assert_eq!(owner.admitted_cursor, applied_cursors(&before).0);
        let AppliedBackend::Segmented { pool, .. } = &owner.backend else {
            panic!("segmented");
        };
        assert_eq!(observe_segmented_parts(pool).unwrap().retained_parts, 0);
    }
    let AppliedBackend::Segmented { pool, .. } = &owner.backend else {
        panic!("segmented");
    };
    let parent = applied_segmented_position(&before).unwrap().applied;
    let identity = SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: hash_segmented_logical_body(&chain.records[0]),
        parent,
        target_height: 1,
        total_length: chain.records[0].len() as u64,
    };
    let layout = plan_segmented_layout(pool, identity).unwrap();
    let first_hold = reserve_segmented_layout(pool, &layout).unwrap();
    let second_hold = reserve_segmented_layout(pool, &layout).unwrap();
    assert!(matches!(
        try_apply_recovery_bytes(&mut owner, &chain.records[0]),
        Err(AppliedError::Segmented(
            crate::persistence::segmented::SegmentedError::Capacity
        ))
    ));
    assert!(Arc::ptr_eq(
        &before,
        &capture_applied_state(&reader).unwrap()
    ));
    assert_eq!(owner.admitted_cursor, applied_cursors(&before).0);
    drop((first_hold, second_hold));
    let reserve = reserve_applied_working(&reader, 1_048_576).unwrap();
    let charged = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes;
    assert!(charged >= 1_048_576);
    drop(reserve);
    assert_eq!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes
            + 1_048_576,
        charged
    );
    drop(finish_applied_state_service(owner));
}

#[test]
fn altered_public_logical_ack_counts_and_bindings_refuse_without_panicking_or_releasing_pending_tail()
 {
    let directory = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (mut owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("segmented"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    let AppliedBackend::Segmented { worker, .. } = &mut owner.backend else {
        panic!("segmented");
    };
    let shutdown = finish_segmented_worker(worker.take().unwrap());
    let PendingPayload::Segmented { ticket, .. } = &owner.pending[0].payload else {
        panic!("segmented pending");
    };
    let ack = try_receive_segmented_ack(ticket).unwrap().unwrap();
    let before = capture_applied_state(&reader).unwrap();
    for variation in 0..5 {
        let mut changed = ack;
        match variation {
            0 => changed.segment_count = 0,
            1 => changed.segment_count = usize::MAX,
            2 => changed.references[0].content_hash[0] ^= 1,
            3 => changed.marker_cursor.content_hash[0] ^= 1,
            _ => changed.target_state_binding[0] ^= 1,
        }
        assert!(
            super::super::validate_segmented_applied_ack(
                &owner.pending[0],
                changed,
                owner.durable_cursor,
                0,
                0
            )
            .is_err()
        );
        assert_eq!(owner.pending.len(), 1);
        assert_eq!(owner.durable_cursor, applied_cursors(&before).1);
        assert!(Arc::ptr_eq(
            &before,
            &capture_applied_state(&reader).unwrap()
        ));
    }
    drop(shutdown);
    drop(finish_applied_state_service(owner));
}

#[test]
fn actual_marker_capacity_failure_keeps_physical_ack_progress_and_all_raw_parts_without_logical_durability()
 {
    let directory = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let mut config = configuration(&directory.path().join("segmented"), &chain);
    config
        .application
        .repository_budget
        .maximum_retained_records = 1;
    let (mut owner, reader) = open_segmented_applied_state_service(config, &chain.genesis)
        .ok()
        .unwrap();
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.repository.is_err());
    assert!(shutdown.acknowledgement_error.is_some());
    let progress = retained_tail_progress(&shutdown.unacknowledged_tail, 0).unwrap();
    assert_eq!(
        (
            progress.logical_target,
            progress.last_acknowledged_physical.sequence
        ),
        (1, 1)
    );
    assert!(progress.complete_marker.is_none());
    assert!(retained_tail_part_bytes(&shutdown.unacknowledged_tail, 0, 0, 0).is_some());
    assert!(retained_tail_part_bytes(&shutdown.unacknowledged_tail, 0, 1, 0).is_some());
    let view = capture_applied_state(&reader).unwrap();
    assert_eq!(
        (
            applied_markers(&view).applied.0,
            applied_markers(&view).durable_recovery.0
        ),
        (1, 0)
    );
    assert!(applied_storage_failed(&view));
}
