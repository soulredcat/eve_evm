// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SealedSegmentedBatch, SegmentedBatchReservation, SegmentedError,
    types::{SEGMENTS, SealedBatch},
};
use eve_storage::records::segmented::{
    SegmentedCommitMarker, SegmentedSegmentMetadata, SegmentedSegmentView, hash_segmented_chunk,
    hash_segmented_logical_body, hash_segmented_marker, write_segmented_marker,
    write_segmented_segment,
};
use eve_storage::records::{OpaqueRecordCursor, prospective_opaque_record_cursor};
use std::sync::Arc;

/// Encode into the already reserved buffers. Prospective references are never sync acknowledgements.
pub fn write_and_seal_segmented_batch(
    mut reservation: SegmentedBatchReservation,
    logical_bytes: &[u8],
    expected: OpaqueRecordCursor,
) -> Result<SealedSegmentedBatch, SegmentedError> {
    let plan = reservation.plan;
    if logical_bytes.len() as u64 != plan.marker.identity.total_length
        || hash_segmented_logical_body(logical_bytes) != plan.marker.identity.logical_id
        || expected.sequence < plan.marker.identity.parent.cursor.sequence
    {
        return Err(SegmentedError::InvalidPlan);
    }
    let pool = Arc::clone(&reservation.metadata.pool);
    let mut cursor = expected;
    let mut references = [OpaqueRecordCursor::default(); SEGMENTS];
    for (index, reference) in references
        .iter_mut()
        .enumerate()
        .take(plan.layout.segment_count)
    {
        let offset = index
            .checked_mul(pool.codec.maximum_chunk_bytes)
            .ok_or(SegmentedError::Overflow)?;
        let end = logical_bytes.len().min(
            offset
                .checked_add(pool.codec.maximum_chunk_bytes)
                .ok_or(SegmentedError::Overflow)?,
        );
        let metadata = SegmentedSegmentMetadata {
            identity: plan.marker.identity,
            index: index as u32,
            count: plan.layout.segment_count as u32,
            offset: offset as u64,
        };
        let data = &logical_bytes[offset..end];
        let chunk_hash = hash_segmented_chunk(&metadata, data, &pool.codec)
            .map_err(|_| SegmentedError::Codec)?;
        let part = reservation.parts[index / plan.layout.per_part]
            .as_mut()
            .ok_or(SegmentedError::InvalidPlan)?;
        let buffer = &mut part.buffers[index % plan.layout.per_part];
        write_segmented_segment(
            &SegmentedSegmentView {
                metadata,
                data,
                chunk_hash,
            },
            buffer,
            &pool.codec,
        )
        .map_err(|_| SegmentedError::Codec)?;
        cursor = prospective_opaque_record_cursor(
            pool.namespace,
            cursor,
            buffer,
            pool.repository.maximum_record_bytes,
        )
        .map_err(|_| SegmentedError::WrongCursor)?;
        *reference = cursor;
    }
    let mut marker_refs = Vec::new();
    marker_refs
        .try_reserve_exact(plan.layout.segment_count)
        .map_err(|_| SegmentedError::Allocation)?;
    if marker_refs.capacity() != plan.layout.segment_count {
        return Err(SegmentedError::UnexpectedCapacity);
    }
    marker_refs.extend_from_slice(&references[..plan.layout.segment_count]);
    let marker_hash = hash_segmented_marker(&plan.marker, &marker_refs, &pool.codec)
        .map_err(|_| SegmentedError::Codec)?;
    let marker = SegmentedCommitMarker {
        metadata: plan.marker,
        references: marker_refs,
        marker_hash,
    };
    let part = reservation.parts[plan.layout.part_count - 1]
        .as_mut()
        .ok_or(SegmentedError::InvalidPlan)?;
    write_segmented_marker(&marker, &mut part.buffers[0], &pool.codec)
        .map_err(|_| SegmentedError::Codec)?;
    let marker_cursor = prospective_opaque_record_cursor(
        pool.namespace,
        cursor,
        &part.buffers[0],
        pool.repository.maximum_record_bytes,
    )
    .map_err(|_| SegmentedError::WrongCursor)?;
    Ok(SealedSegmentedBatch(Arc::new(SealedBatch {
        plan,
        id: reservation.id,
        parts: reservation.parts,
        _metadata: reservation.metadata,
        expected,
        marker_cursor,
        references,
    })))
}
