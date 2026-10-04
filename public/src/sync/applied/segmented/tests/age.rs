// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{configuration, logical_chain};
use crate::persistence::segmented::{
    SegmentedError, bind_segmented_reservation, observe_segmented_parts, plan_segmented_layout,
    reserve_segmented_layout, segmented_batch_marker_cursor, write_and_seal_segmented_batch,
};
use crate::sync::applied::*;
use crate::sync::applied::{
    publication::build_applied_markers,
    recovery::prepare_charged_import,
    resources::reserve_estimated_working,
    state::{AppliedState, applied_state_commit},
    types::{AppliedBackend, AppliedPublication},
};
use eve_storage::records::segmented::{
    SegmentedLogicalIdentity, SegmentedRecoveryAnchor, SegmentedRecoveryMode,
    hash_segmented_logical_body,
};
use std::{sync::Arc, thread, time::Duration};

#[test]
fn final_segmented_admission_age_refusal_after_actual_candidate_preparation_releases_all_parts_and_state()
 {
    let directory = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let mut config = configuration(&directory.path().join("segmented"), &chain);
    config.policy.maximum_queue_age_ms = 1;
    let (mut owner, reader) = open_segmented_applied_state_service(config, &chain.genesis)
        .ok()
        .unwrap();
    let original = capture_applied_state(&reader).unwrap();
    let before_charge = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes;
    let mut position = applied_segmented_position(&original).unwrap();
    let AppliedBackend::Segmented { pool, .. } = &owner.backend else {
        panic!("segmented");
    };
    let pool = Arc::clone(pool);
    let identity = SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: hash_segmented_logical_body(&chain.records[0]),
        parent: position.applied,
        target_height: 1,
        total_length: chain.records[0].len() as u64,
    };
    let layout = plan_segmented_layout(&pool, identity).unwrap();
    let unbound = reserve_segmented_layout(&pool, &layout).unwrap();
    let AppliedState::AuthenticatedImport(parent) = &original.generation.state else {
        panic!("imported");
    };
    let generation = prepare_charged_import(
        parent,
        &chain.records[0],
        &owner.config.state_budget,
        &owner.reader.working,
        true,
    )
    .unwrap();
    let scratch = reserve_estimated_working(
        &reader.working,
        eve_state::BOUNDED_STATE_CODEC_SCRATCH_BYTES,
    )
    .unwrap();
    let target = applied_state_commit(&generation.state).target.clone();
    let binding = super::super::bind_local_state_version(&target).unwrap();
    let bound = bind_segmented_reservation(unbound, binding).ok().unwrap();
    let batch =
        write_and_seal_segmented_batch(bound, &chain.records[0], owner.admitted_cursor).unwrap();
    let cursor = segmented_batch_marker_cursor(&batch);
    position.applied = SegmentedRecoveryAnchor {
        height: 1,
        cursor,
        state_binding: binding,
    };
    let markers = build_applied_markers(&generation.state, 0).unwrap();
    let publication = Arc::new(AppliedPublication {
        generation,
        markers,
        admitted_cursor: cursor,
        durable_cursor: owner.durable_cursor,
        storage_failed: false,
        segmented_position: Some(position),
    });
    drop(scratch);
    thread::sleep(Duration::from_millis(5));
    assert!(matches!(
        super::super::admit_segmented_publication::admit_segmented_publication(
            &mut owner,
            publication,
            batch,
            identity,
            binding,
            target
        ),
        Err(AppliedError::Segmented(SegmentedError::QueueAged))
    ));
    assert!(Arc::ptr_eq(
        &original,
        &capture_applied_state(&reader).unwrap()
    ));
    assert!(owner.pending.is_empty());
    assert_eq!(owner.admitted_cursor, applied_cursors(&original).0);
    assert_eq!(observe_segmented_parts(&pool).unwrap().retained_parts, 0);
    assert_eq!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes,
        before_charge
    );
    drop(finish_applied_state_service(owner));
}
