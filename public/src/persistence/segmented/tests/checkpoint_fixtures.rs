// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{checkpoints::*, *};
use eve_state::{
    StateVersion, build_state_version, development_state_budget, initialize_development_state,
};
use eve_storage::records::{
    OpaqueRecordCursor,
    segmented::{SegmentedRecoveryAnchor, checkpoints::*},
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

pub(super) fn limits() -> CheckpointBaseLimits {
    CheckpointBaseLimits {
        maximum_payload_bytes: CHECKPOINT_BASE_MAX_PAYLOAD_BYTES,
    }
}
pub(super) fn version(height: u64) -> StateVersion {
    let genesis = eve_development_fixtures::genesis::genesis();
    let budget = development_state_budget();
    let initial = initialize_development_state(&genesis, &budget).unwrap();
    let mut header = initial.block.header.clone();
    header.number = height;
    build_state_version(&initial.state, &header, &budget).unwrap()
}
pub(super) fn metadata(
    parent: SegmentedRecoveryAnchor,
    physical: OpaqueRecordCursor,
    height: u64,
) -> CheckpointBaseMetadata {
    CheckpointBaseMetadata {
        mode: CheckpointBaseMode::AuthenticatedImport,
        previous_opaque_cursor: physical,
        previous_logical_anchor: parent,
        target_height: height,
        target_state_binding: [5; 32],
        snapshot_manifest_id: [6; 32],
        snapshot_body_hash: [7; 32],
        proof_manifest_id: [8; 32],
        proof_root: [9; 32],
        proof_genesis_height: 0,
        execution_start: 1,
        execution_end: height,
        lookahead_height: height + 1,
        retained_start: 0,
        retained_end: height,
    }
}
pub(super) fn record(
    pool: &Arc<SegmentedPartPool>,
    parent: SegmentedRecoveryAnchor,
    physical: OpaqueRecordCursor,
    height: u64,
) -> SealedCheckpointRecord {
    seal_checkpoint_base_record(
        pool,
        metadata(parent, physical, height),
        &version(height),
        limits(),
    )
    .unwrap()
}
pub(super) fn wait_checkpoint(ticket: &CheckpointTicket) -> CheckpointAck {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(ack) = try_receive_checkpoint_ack(ticket).unwrap() {
            return ack;
        }
        assert!(
            Instant::now() < deadline,
            "checkpoint ACK deadline exceeded"
        );
        std::thread::yield_now();
    }
}
pub(super) fn wait_batch(ticket: &SegmentedTicket) -> SegmentedLogicalAck {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(ack) = try_receive_segmented_ack(ticket).unwrap() {
            return ack;
        }
        assert!(Instant::now() < deadline, "batch ACK deadline exceeded");
        std::thread::yield_now();
    }
}
