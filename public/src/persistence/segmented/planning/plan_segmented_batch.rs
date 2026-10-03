// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedBatchPlan, SegmentedError, SegmentedPartPool,
    types::{PARTS, SEGMENTS},
};
use eve_storage::records::segmented::{SegmentedLogicalIdentity, SegmentedMarkerMetadata};
pub fn plan_segmented_batch(
    pool: &SegmentedPartPool,
    identity: SegmentedLogicalIdentity,
    target_state_binding: [u8; 32],
) -> Result<SegmentedBatchPlan, SegmentedError> {
    let length = usize::try_from(identity.total_length).map_err(|_| SegmentedError::Overflow)?;
    if length == 0
        || length > pool.codec.maximum_logical_bytes
        || identity.logical_id == [0; 32]
        || identity.parent.state_binding == [0; 32]
        || identity.parent.cursor.content_hash == [0; 32]
        || (identity.parent.height == 0) != (identity.parent.cursor.sequence == 0)
        || identity.parent.height.checked_add(1) != Some(identity.target_height)
        || target_state_binding == [0; 32]
    {
        return Err(SegmentedError::InvalidPlan);
    }
    let count = length.div_ceil(pool.codec.maximum_chunk_bytes);
    let per_part = usize::try_from(pool.policy.maximum_segments_per_part)
        .map_err(|_| SegmentedError::Overflow)?;
    let part_count = count
        .div_ceil(per_part)
        .checked_add(1)
        .ok_or(SegmentedError::Overflow)?;
    if count > SEGMENTS || count > pool.codec.maximum_segments || part_count > PARTS {
        return Err(SegmentedError::Capacity);
    }
    let mut plan = SegmentedBatchPlan {
        namespace: pool.namespace,
        codec: pool.codec,
        per_part,
        marker: SegmentedMarkerMetadata {
            identity,
            target_state_binding,
        },
        segment_count: count,
        part_count,
        segment_lengths: [0; SEGMENTS],
        part_lengths: [[0; 2]; PARTS],
        total_encoded: 0,
    };
    for index in 0..count {
        let offset = index
            .checked_mul(pool.codec.maximum_chunk_bytes)
            .ok_or(SegmentedError::Overflow)?;
        let bytes = (length - offset)
            .min(pool.codec.maximum_chunk_bytes)
            .checked_add(209)
            .ok_or(SegmentedError::Overflow)?;
        plan.segment_lengths[index] = bytes;
        plan.part_lengths[index / per_part][index % per_part] = bytes;
        plan.total_encoded = plan
            .total_encoded
            .checked_add(bytes)
            .ok_or(SegmentedError::Overflow)?;
    }
    let marker = count
        .checked_mul(40)
        .and_then(|bytes| bytes.checked_add(225))
        .ok_or(SegmentedError::Overflow)?;
    plan.part_lengths[part_count - 1][0] = marker;
    plan.total_encoded = plan
        .total_encoded
        .checked_add(marker)
        .ok_or(SegmentedError::Overflow)?;
    Ok(plan)
}
