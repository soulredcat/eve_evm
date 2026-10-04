// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        SEGMENTED_SEGMENT_OVERHEAD_BYTES, SegmentedCodecError, SegmentedCodecLimits,
        SegmentedSegment, SegmentedSegmentView,
        validation::validate_segmented_segment::validate_segmented_segment,
    },
    write_segmented_segment::write_segmented_segment,
};

/// Convenience encoder; callers must separately charge this allocated output.
pub fn encode_segmented_segment(
    segment: &SegmentedSegment,
    limits: &SegmentedCodecLimits,
) -> Result<Vec<u8>, SegmentedCodecError> {
    validate_segmented_segment(&segment.metadata, segment.data.len(), limits)?;
    let size = segment
        .data
        .len()
        .checked_add(SEGMENTED_SEGMENT_OVERHEAD_BYTES)
        .ok_or(SegmentedCodecError::LimitExceeded)?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(size)
        .map_err(|_| SegmentedCodecError::AllocationFailed)?;
    write_segmented_segment(
        &SegmentedSegmentView {
            metadata: segment.metadata,
            data: &segment.data,
            chunk_hash: segment.chunk_hash,
        },
        &mut output,
        limits,
    )?;
    Ok(output)
}
