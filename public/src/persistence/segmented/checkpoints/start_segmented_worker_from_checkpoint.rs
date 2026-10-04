// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedError, SegmentedPartPool, SegmentedWorker,
    pool::acquire_segmented_worker,
    worker::{
        create_segmented_worker_state::create_segmented_worker_state,
        spawn_segmented_worker::spawn_segmented_worker,
    },
};
use eve_storage::records::{
    OpaqueRecordCursor, OpaqueRecordRepository, opaque_record_budget, opaque_record_cursor,
    opaque_record_identity,
    segmented::{
        SegmentedRecoveryAnchor,
        checkpoints::{
            CheckpointBaseLimits, CheckpointBaseMembership, PreparedCheckpointBaseTarget,
            checkpoint_base_membership_record, checkpoint_base_membership_view,
            read_checkpoint_base_membership, required_checkpoint_base_membership_reservation,
        },
    },
};
use std::sync::Arc;

/// Separate startup requiring actual stored checkpoint-base membership.
/// No snapshot artifact, proof, state or freshness authority is granted here.
pub fn start_segmented_worker_from_checkpoint(
    repository: OpaqueRecordRepository,
    pool: Arc<SegmentedPartPool>,
    membership: &CheckpointBaseMembership,
    target: &PreparedCheckpointBaseTarget,
    limits: CheckpointBaseLimits,
) -> Result<SegmentedWorker, SegmentedError> {
    let worker_lifetime = acquire_segmented_worker(&pool)?;
    if opaque_record_identity(&repository) != pool.namespace
        || opaque_record_budget(&repository) != pool.repository
    {
        return Err(SegmentedError::ForeignPool);
    }
    let cursor = opaque_record_cursor(&repository).map_err(|_| SegmentedError::StorageFailed)?;
    let row = checkpoint_base_membership_record(membership);
    let base_cursor = OpaqueRecordCursor {
        sequence: row.sequence,
        content_hash: row.content_hash,
    };
    if base_cursor.sequence > cursor.sequence {
        return Err(SegmentedError::WrongCursor);
    }
    let metadata = checkpoint_base_membership_view(membership).metadata;
    let logical = SegmentedRecoveryAnchor {
        height: metadata.target_height,
        cursor: base_cursor,
        state_binding: metadata.target_state_binding,
    };
    let state = create_segmented_worker_state(pool, cursor, logical, worker_lifetime)?;
    let required = required_checkpoint_base_membership_reservation(&state.pool.repository, &limits)
        .map_err(|_| SegmentedError::InvalidConfiguration)?;
    if u64::try_from(required).map_err(|_| SegmentedError::Overflow)?
        > state.pool.policy.maximum_scratch_bytes
    {
        return Err(SegmentedError::Capacity);
    }
    // No live writer exists yet; the same real worker scratch accounting bounds this read.
    state
        .scratch
        .store(required as u64, std::sync::atomic::Ordering::Release);
    let checked = read_checkpoint_base_membership(
        &repository,
        base_cursor,
        metadata,
        target,
        &limits,
        required,
    )
    .map_err(|_| SegmentedError::WrongCursor)?;
    if checkpoint_base_membership_record(&checked).payload != row.payload {
        return Err(SegmentedError::WrongCursor);
    }
    drop(checked);
    state.scratch.store(0, std::sync::atomic::Ordering::Release);
    spawn_segmented_worker(repository, state)
}
