// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SEGMENTED_MARKER_OVERHEAD_BYTES, SegmentedCodecError, SegmentedCodecLimits,
    SegmentedCommitMarker, framing::encode_segmented_marker_header::encode_segmented_marker_header,
    hash_segmented_marker, validation::validate_segmented_marker::validate_segmented_marker,
};

/// No output allocation or growth. Marker references need their own bounded
/// metadata charge; this local encoding grants no sync or complete-height claim.
pub fn write_segmented_marker(
    marker: &SegmentedCommitMarker,
    output: &mut Vec<u8>,
    limits: &SegmentedCodecLimits,
) -> Result<(), SegmentedCodecError> {
    validate_segmented_marker(&marker.metadata, &marker.references, limits)?;
    let size = marker
        .references
        .len()
        .checked_mul(40)
        .and_then(|bytes| bytes.checked_add(SEGMENTED_MARKER_OVERHEAD_BYTES))
        .ok_or(SegmentedCodecError::LimitExceeded)?;
    if !output.is_empty() || output.capacity() != size {
        return Err(SegmentedCodecError::OutputCapacity);
    }
    let hash = hash_segmented_marker(&marker.metadata, &marker.references, limits)?;
    if hash != marker.marker_hash {
        return Err(SegmentedCodecError::HashMismatch);
    }
    let count =
        u32::try_from(marker.references.len()).map_err(|_| SegmentedCodecError::LimitExceeded)?;
    output.extend_from_slice(&encode_segmented_marker_header(&marker.metadata, count));
    for reference in &marker.references {
        output.extend_from_slice(&reference.sequence.to_be_bytes());
        output.extend_from_slice(&reference.content_hash);
    }
    output.extend_from_slice(&marker.marker_hash);
    Ok(())
}
