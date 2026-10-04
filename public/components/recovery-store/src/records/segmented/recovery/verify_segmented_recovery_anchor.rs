// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::SegmentedRecoveryError;
use crate::records::{
    OpaqueRecordRepository, opaque_record_bootstrap_cursor, opaque_record_cursor,
    read_opaque_record,
    segmented::{
        SegmentedCodecLimits, SegmentedRecoveryAnchor, hash_segmented_logical_body,
        preflight_segmented_record, segmented_marker_reference_at, segmented_marker_view,
        segmented_segment_view,
    },
};

/// At most one real marker plus six referenced rows. No prior-history Vec or
/// certificate validation. The caller's reusable logical buffer stays charged.
pub(super) fn verify_segmented_recovery_anchor(
    repository: &OpaqueRecordRepository,
    anchor: SegmentedRecoveryAnchor,
    limits: &SegmentedCodecLimits,
    body: &mut Vec<u8>,
) -> Result<(), SegmentedRecoveryError> {
    if anchor.state_binding == [0; 32] {
        return Err(SegmentedRecoveryError::InvalidAnchor);
    }
    let head =
        opaque_record_cursor(repository).map_err(|_| SegmentedRecoveryError::StorageFailed)?;
    if anchor.height == 0 {
        if anchor.cursor != opaque_record_bootstrap_cursor(repository) {
            return Err(SegmentedRecoveryError::InvalidAnchor);
        }
        return Ok(());
    }
    if anchor.cursor.sequence == 0 || anchor.cursor.sequence > head.sequence {
        return Err(SegmentedRecoveryError::InvalidAnchor);
    }
    let record = read_opaque_record(repository, anchor.cursor.sequence)
        .map_err(|_| SegmentedRecoveryError::StorageFailed)?
        .ok_or(SegmentedRecoveryError::InvalidAnchor)?;
    if record.content_hash != anchor.cursor.content_hash {
        return Err(SegmentedRecoveryError::InvalidAnchor);
    }
    let preflight = preflight_segmented_record(&record.payload, limits)
        .map_err(SegmentedRecoveryError::Codec)?;
    let marker = segmented_marker_view(&preflight).ok_or(SegmentedRecoveryError::InvalidAnchor)?;
    if marker.metadata.identity.target_height != anchor.height
        || marker.metadata.target_state_binding != anchor.state_binding
    {
        return Err(SegmentedRecoveryError::InvalidAnchor);
    }
    let mut previous = None;
    for index in 0..marker.reference_count {
        let reference =
            segmented_marker_reference_at(&marker, index).map_err(SegmentedRecoveryError::Codec)?;
        if reference.sequence >= record.sequence {
            return Err(SegmentedRecoveryError::InvalidMarker);
        }
        let segment_record = read_opaque_record(repository, reference.sequence)
            .map_err(|_| SegmentedRecoveryError::StorageFailed)?
            .ok_or(SegmentedRecoveryError::InvalidMarker)?;
        if segment_record.content_hash != reference.content_hash
            || previous.is_some_and(|cursor| segment_record.parent != cursor)
        {
            return Err(SegmentedRecoveryError::InvalidMarker);
        }
        let preflight = preflight_segmented_record(&segment_record.payload, limits)
            .map_err(SegmentedRecoveryError::Codec)?;
        let segment =
            segmented_segment_view(&preflight).ok_or(SegmentedRecoveryError::InvalidMarker)?;
        if segment.metadata.identity != marker.metadata.identity
            || segment.metadata.index as usize != index
            || segment.metadata.count as usize != marker.reference_count
            || segment.metadata.offset as usize != body.len()
        {
            return Err(SegmentedRecoveryError::InvalidMarker);
        }
        body.extend_from_slice(segment.data);
        previous = Some(reference);
    }
    if previous != Some(record.parent)
        || record.parent.sequence.checked_add(1) != Some(record.sequence)
        || body.len() as u64 != marker.metadata.identity.total_length
    {
        return Err(SegmentedRecoveryError::InvalidMarker);
    }
    if hash_segmented_logical_body(body.as_slice()) != marker.metadata.identity.logical_id {
        return Err(SegmentedRecoveryError::LogicalHashMismatch);
    }
    Ok(())
}
