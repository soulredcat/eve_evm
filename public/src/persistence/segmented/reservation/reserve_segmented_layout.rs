// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedBatchLayout, SegmentedError, SegmentedPartPool, UnboundSegmentedReservation,
    pool::estimate_segmented_metadata,
    types::{AllocatedPart, MetadataLease, PARTS, PartLease, PartSlot},
};
use std::{sync::Arc, time::Instant};

/// Reserve all part slots/bytes and distinct metadata atomically, before any Vec allocation.
pub fn reserve_segmented_layout(
    pool: &Arc<SegmentedPartPool>,
    plan: &SegmentedBatchLayout,
) -> Result<UnboundSegmentedReservation, SegmentedError> {
    if plan.namespace != pool.namespace
        || plan.codec != pool.codec
        || plan.per_part as u64 != pool.policy.maximum_segments_per_part
    {
        return Err(SegmentedError::ForeignPool);
    }
    let (_, metadata_bytes, _) = estimate_segmented_metadata();
    let mut leases: [Option<PartLease>; PARTS] = std::array::from_fn(|_| None);
    let (id, metadata) = {
        let mut state = pool
            .accounting
            .lock()
            .map_err(|_| SegmentedError::AccountingUnavailable)?;
        let next_bytes = state
            .bytes
            .checked_add(plan.total_encoded as u64)
            .ok_or(SegmentedError::Overflow)?;
        let next_metadata = state
            .metadata
            .checked_add(metadata_bytes as u64)
            .ok_or(SegmentedError::Overflow)?;
        let count = state
            .slots
            .iter()
            .flatten()
            .count()
            .checked_add(plan.part_count)
            .ok_or(SegmentedError::Overflow)?;
        if next_bytes > pool.policy.queue_bytes
            || next_metadata > pool.policy.maximum_metadata_bytes
            || count > pool.policy.retained_parts as usize
        {
            return Err(SegmentedError::Capacity);
        }
        for lengths in plan.part_lengths.iter().take(plan.part_count) {
            let bytes = lengths[0]
                .checked_add(lengths[1])
                .ok_or(SegmentedError::Overflow)?;
            if bytes as u64 > pool.policy.maximum_part_bytes {
                return Err(SegmentedError::Capacity);
            }
        }
        let id = state
            .next_id
            .checked_add(1)
            .ok_or(SegmentedError::Overflow)?;
        state.next_id = id;
        state.bytes = next_bytes;
        state.metadata = next_metadata;
        let mut part = 0;
        for slot in 0..PARTS {
            if part == plan.part_count {
                break;
            }
            if state.slots[slot].is_none() {
                let bytes = plan.part_lengths[part][0] + plan.part_lengths[part][1];
                state.slots[slot] = Some(PartSlot {
                    id,
                    bytes: bytes as u64,
                    created: Instant::now(),
                });
                leases[part] = Some(PartLease {
                    pool: Arc::clone(pool),
                    slot,
                    id,
                });
                part += 1;
            }
        }
        (
            id,
            MetadataLease {
                pool: Arc::clone(pool),
                bytes: metadata_bytes as u64,
            },
        )
    };
    let mut parts: [Option<AllocatedPart>; PARTS] = std::array::from_fn(|_| None);
    for index in 0..plan.part_count {
        let mut buffers: [Vec<u8>; 2] = std::array::from_fn(|_| Vec::new());
        for (buffer, capacity) in buffers.iter_mut().zip(plan.part_lengths[index]) {
            buffer
                .try_reserve_exact(capacity)
                .map_err(|_| SegmentedError::Allocation)?;
            if buffer.capacity() != capacity {
                return Err(SegmentedError::UnexpectedCapacity);
            }
        }
        parts[index] = Some(AllocatedPart {
            buffers,
            _lease: leases[index].take().ok_or(SegmentedError::InvalidPlan)?,
        });
    }
    Ok(UnboundSegmentedReservation {
        layout: *plan,
        id,
        parts,
        metadata,
    })
}
