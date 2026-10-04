// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SEGMENTED_MARKER_OVERHEAD_BYTES, SEGMENTED_MAX_CHUNK_BYTES,
    SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES, SEGMENTED_MAX_REFERENCES,
    SEGMENTED_SEGMENT_OVERHEAD_BYTES, SegmentedCodecError, SegmentedCodecLimits,
};

pub fn validate_segmented_codec_limits(
    limits: &SegmentedCodecLimits,
) -> Result<(), SegmentedCodecError> {
    let invalid = SegmentedCodecError::InvalidLimits;
    if limits.maximum_logical_bytes == 0
        || limits.maximum_chunk_bytes == 0
        || limits.maximum_segments == 0
        || limits.maximum_segments > SEGMENTED_MAX_REFERENCES
        || limits.maximum_chunk_bytes > SEGMENTED_MAX_CHUNK_BYTES
        || limits.maximum_payload_bytes > SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES
    {
        return Err(invalid);
    }
    let capacity = limits
        .maximum_chunk_bytes
        .checked_mul(limits.maximum_segments)
        .ok_or(invalid)?;
    let segment = limits
        .maximum_chunk_bytes
        .checked_add(SEGMENTED_SEGMENT_OVERHEAD_BYTES)
        .ok_or(invalid)?;
    let marker = limits
        .maximum_segments
        .checked_mul(40)
        .and_then(|bytes| bytes.checked_add(SEGMENTED_MARKER_OVERHEAD_BYTES))
        .ok_or(invalid)?;
    if limits.maximum_logical_bytes > capacity
        || segment > limits.maximum_payload_bytes
        || marker > limits.maximum_payload_bytes
    {
        return Err(invalid);
    }
    Ok(())
}
