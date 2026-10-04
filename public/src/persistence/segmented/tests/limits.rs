// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::*;
use super::super::{pool::estimate_segmented_metadata, worker::required_segment_scratch};
use super::{
    fixtures::{batch, fixture},
    pause,
};
use eve_node_policy::{
    SegmentedRecoveryBounds, available_segmented_allowance, development_public_budget,
    validate_segmented_recovery_policy,
};
use eve_storage::records::segmented::SegmentedRecoveryAnchor;
use std::{sync::Arc, time::Duration};

#[test]
fn numerically_valid_lower_segment_policy_cannot_use_a_different_codec_payload_ceiling() {
    let fixture = fixture();
    let base = development_public_budget();
    let mut codec = fixture.pool.codec;
    codec.maximum_logical_bytes = 512 * 1_024;
    let mut policy = fixture.pool.policy;
    policy.maximum_segment_bytes = 1_048_576;
    let (pool, batch, worker) = estimate_segmented_metadata();
    let bounds = SegmentedRecoveryBounds {
        maximum_logical_bytes: codec.maximum_logical_bytes as u64,
        maximum_segment_bytes: codec.maximum_payload_bytes as u64,
        segment_header_bytes: 177,
        segment_footer_bytes: 32,
        maximum_segments: 6,
        maximum_marker_bytes: 465,
        opaque_record_header_bytes: 88,
        maximum_opaque_record_bytes: fixture.budget.maximum_record_bytes as u64,
        maximum_opaque_read_bytes: fixture.budget.maximum_read_bytes as u64,
        maximum_opaque_batch_bytes: fixture.budget.maximum_batch_bytes as u64,
        maximum_opaque_batch_records: 1,
        required_metadata_bytes: (pool + 2 * batch + worker) as u64,
        required_scratch_bytes: required_segment_scratch(
            codec.maximum_payload_bytes,
            fixture.budget,
        )
        .unwrap(),
        metadata_limit_bytes: policy.maximum_metadata_bytes,
        scratch_limit_bytes: policy.maximum_scratch_bytes,
        available_auxiliary_bytes: available_segmented_allowance(base).unwrap(),
    };
    assert!(validate_segmented_recovery_policy(base, policy, bounds).is_ok());
    let before = observe_segmented_parts(&fixture.pool).unwrap();
    assert!(matches!(
        create_segmented_part_pool(base, policy, codec, fixture.budget, fixture.namespace),
        Err(SegmentedError::InvalidConfiguration)
    ));
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_encoded_bytes,
        before.retained_encoded_bytes
    );
}

#[test]
fn logical_lag_one_refuses_second_batch_while_first_real_sync_is_paused() {
    let mut fixture = fixture();
    Arc::get_mut(&mut fixture.pool)
        .unwrap()
        .policy
        .maximum_logical_lag_blocks = 1;
    let first = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"first",
    );
    let next = SegmentedRecoveryAnchor {
        height: 1,
        cursor: first.0.marker_cursor,
        state_binding: [8; 32],
    };
    let second = batch(&fixture.pool, next, next.cursor, b"second");
    let worker = start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("valid worker"));
    let (entered, resume) = pause(&worker, 0, false);
    let ticket = try_submit_segmented_batch(&worker, fixture.parent.cursor, first)
        .ok()
        .unwrap();
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    let rejected = try_submit_segmented_batch(&worker, next.cursor, second)
        .err()
        .unwrap();
    let position = worker.state.admission.lock().unwrap().cursor;
    resume.send(()).unwrap();
    assert_eq!(rejected.error, SegmentedError::LogicalLag);
    assert_eq!(position, next.cursor);
    let shutdown = finish_segmented_worker(worker);
    assert_eq!(
        try_receive_segmented_ack(&ticket)
            .unwrap()
            .unwrap()
            .identity
            .target_height,
        1
    );
    drop(rejected);
    drop(shutdown);
}
