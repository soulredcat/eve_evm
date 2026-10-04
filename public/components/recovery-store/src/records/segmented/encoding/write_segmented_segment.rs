// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SEGMENTED_SEGMENT_OVERHEAD_BYTES, SegmentedCodecError, SegmentedCodecLimits,
    SegmentedSegmentView,
    framing::encode_segmented_segment_header::encode_segmented_segment_header,
    hash_segmented_chunk, validation::validate_segmented_segment::validate_segmented_segment,
};

/// Write directly into the caller's empty, exact-capacity leased Vec, without an
/// intermediate encoded payload or growth. Failure leaves the buffer unchanged.
pub fn write_segmented_segment(
    view: &SegmentedSegmentView<'_>,
    output: &mut Vec<u8>,
    limits: &SegmentedCodecLimits,
) -> Result<(), SegmentedCodecError> {
    validate_segmented_segment(&view.metadata, view.data.len(), limits)?;
    let size = view
        .data
        .len()
        .checked_add(SEGMENTED_SEGMENT_OVERHEAD_BYTES)
        .ok_or(SegmentedCodecError::LimitExceeded)?;
    if !output.is_empty() || output.capacity() != size {
        return Err(SegmentedCodecError::OutputCapacity);
    }
    let hash = hash_segmented_chunk(&view.metadata, view.data, limits)?;
    if hash != view.chunk_hash {
        return Err(SegmentedCodecError::HashMismatch);
    }
    let length = u32::try_from(view.data.len()).map_err(|_| SegmentedCodecError::LimitExceeded)?;
    output.extend_from_slice(&encode_segmented_segment_header(&view.metadata, length));
    output.extend_from_slice(view.data);
    output.extend_from_slice(&view.chunk_hash);
    Ok(())
}
