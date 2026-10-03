// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{SegmentedError, types::WorkerState},
    reserve_scratch::reserve_scratch,
};
use eve_storage::records::segmented::{
    SegmentedRecoveryAnchor, preflight_segmented_record, segmented_marker_reference_at,
    segmented_marker_view, segmented_segment_view,
};
use eve_storage::records::{OpaqueRecordRepository, opaque_record_cursor, read_opaque_record};
use std::sync::Arc;

/// Local marker/segment membership only; assembled body identity and certificates remain upstream.
pub(super) fn verify_startup_marker_membership(
    repository: &OpaqueRecordRepository,
    state: &Arc<WorkerState>,
    parent: SegmentedRecoveryAnchor,
) -> Result<(), SegmentedError> {
    let _scratch = reserve_scratch(state, state.pool.codec.maximum_payload_bytes)?;
    let head = opaque_record_cursor(repository).map_err(|_| SegmentedError::StorageFailed)?;
    let record = read_opaque_record(repository, parent.cursor.sequence)
        .map_err(|_| SegmentedError::StorageFailed)?
        .ok_or(SegmentedError::WrongCursor)?;
    if record.content_hash != parent.cursor.content_hash
        || record.sequence != parent.cursor.sequence
    {
        return Err(SegmentedError::WrongCursor);
    }
    let preflight = preflight_segmented_record(&record.payload, &state.pool.codec)
        .map_err(|_| SegmentedError::Codec)?;
    let marker = segmented_marker_view(&preflight).ok_or(SegmentedError::WrongCursor)?;
    if marker.metadata.identity.target_height != parent.height
        || marker.metadata.target_state_binding != parent.state_binding
        || marker.reference_count == 0
        || marker.reference_count > state.pool.codec.maximum_segments
    {
        return Err(SegmentedError::WrongCursor);
    }
    let last = segmented_marker_reference_at(&marker, marker.reference_count - 1)
        .map_err(|_| SegmentedError::Codec)?;
    if last.sequence.checked_add(1) != Some(record.sequence) {
        return Err(SegmentedError::WrongCursor);
    }
    let mut previous = None;
    for index in 0..marker.reference_count {
        let reference =
            segmented_marker_reference_at(&marker, index).map_err(|_| SegmentedError::Codec)?;
        if reference.sequence > head.sequence
            || reference.sequence >= record.sequence
            || previous
                .is_some_and(|sequence: u64| sequence.checked_add(1) != Some(reference.sequence))
        {
            return Err(SegmentedError::WrongCursor);
        }
        let segment_record = read_opaque_record(repository, reference.sequence)
            .map_err(|_| SegmentedError::StorageFailed)?
            .ok_or(SegmentedError::WrongCursor)?;
        if segment_record.sequence != reference.sequence
            || segment_record.content_hash != reference.content_hash
        {
            return Err(SegmentedError::WrongCursor);
        }
        let segment_preflight =
            preflight_segmented_record(&segment_record.payload, &state.pool.codec)
                .map_err(|_| SegmentedError::Codec)?;
        let segment =
            segmented_segment_view(&segment_preflight).ok_or(SegmentedError::WrongCursor)?;
        let offset = index
            .checked_mul(state.pool.codec.maximum_chunk_bytes)
            .ok_or(SegmentedError::Overflow)?;
        if segment.metadata.identity != marker.metadata.identity
            || segment.metadata.index as usize != index
            || segment.metadata.count as usize != marker.reference_count
            || segment.metadata.offset != offset as u64
        {
            return Err(SegmentedError::WrongCursor);
        }
        previous = Some(reference.sequence);
    }
    Ok(())
}
