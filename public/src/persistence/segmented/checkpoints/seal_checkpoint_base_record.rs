// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedError, SegmentedPartPool, pool::reserve_metadata};
use super::types::{CheckpointRecord, SealedCheckpointRecord};
use eve_state::StateVersion;
use eve_storage::records::{
    prospective_opaque_record_cursor,
    segmented::checkpoints::{
        CheckpointBaseLimits, CheckpointBaseMetadata, encode_checkpoint_base,
        prepare_checkpoint_base_target, required_checkpoint_base_encoding_reservation,
    },
};
use std::{mem::size_of, sync::Arc, time::Instant};

/// Reserve actual pool charge before canonical target or payload allocations.
/// Caller verifies every referenced snapshot/proof before requesting activation.
pub fn seal_checkpoint_base_record(
    pool: &Arc<SegmentedPartPool>,
    metadata: CheckpointBaseMetadata,
    version: &StateVersion,
    limits: CheckpointBaseLimits,
) -> Result<SealedCheckpointRecord, SegmentedError> {
    let required = required_checkpoint_base_encoding_reservation(&limits)
        .map_err(|_| SegmentedError::InvalidConfiguration)?;
    let charged = required
        .checked_add(size_of::<CheckpointRecord>())
        .and_then(|bytes| bytes.checked_add(4_096))
        .ok_or(SegmentedError::Overflow)?;
    let lease = reserve_metadata(pool, charged)?;
    let id = {
        let mut accounting = pool
            .accounting
            .lock()
            .map_err(|_| SegmentedError::AccountingUnavailable)?;
        let id = accounting.next_id;
        accounting.next_id = id.checked_add(1).ok_or(SegmentedError::Overflow)?;
        id
    };
    let created = Instant::now();
    let target = prepare_checkpoint_base_target(version, &limits, required)
        .map_err(|_| SegmentedError::Codec)?;
    let payload = encode_checkpoint_base(metadata, &target, &limits, required)
        .map_err(|_| SegmentedError::Codec)?;
    if payload.capacity() > limits.maximum_payload_bytes {
        return Err(SegmentedError::UnexpectedCapacity);
    }
    let cursor = prospective_opaque_record_cursor(
        pool.namespace,
        metadata.previous_opaque_cursor,
        &payload,
        pool.repository.maximum_record_bytes,
    )
    .map_err(|_| SegmentedError::Codec)?;
    if payload
        .len()
        .checked_add(88)
        .ok_or(SegmentedError::Overflow)?
        > pool.repository.maximum_batch_bytes
    {
        return Err(SegmentedError::Capacity);
    }
    Ok(SealedCheckpointRecord(Arc::new(CheckpointRecord {
        payload,
        metadata,
        cursor,
        id,
        created,
        _metadata: lease,
    })))
}
