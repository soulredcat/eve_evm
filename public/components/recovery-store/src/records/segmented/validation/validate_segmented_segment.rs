// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{SegmentedCodecError, SegmentedCodecLimits, SegmentedSegmentMetadata},
    validate_segmented_codec_limits::validate_segmented_codec_limits,
    validate_segmented_identity::validate_segmented_identity,
};

pub(in crate::records::segmented) fn validate_segmented_segment(
    metadata: &SegmentedSegmentMetadata,
    data_length: usize,
    limits: &SegmentedCodecLimits,
) -> Result<(), SegmentedCodecError> {
    validate_segmented_codec_limits(limits)?;
    let count = validate_segmented_identity(&metadata.identity, limits)?;
    let index = usize::try_from(metadata.index).map_err(|_| SegmentedCodecError::InvalidChunk)?;
    if usize::try_from(metadata.count).ok() != Some(count) || index >= count {
        return Err(SegmentedCodecError::InvalidChunk);
    }
    let offset = index
        .checked_mul(limits.maximum_chunk_bytes)
        .ok_or(SegmentedCodecError::InvalidChunk)?;
    let total = usize::try_from(metadata.identity.total_length)
        .map_err(|_| SegmentedCodecError::InvalidChunk)?;
    let expected = total
        .checked_sub(offset)
        .ok_or(SegmentedCodecError::InvalidChunk)?
        .min(limits.maximum_chunk_bytes);
    if u64::try_from(offset).ok() != Some(metadata.offset) || data_length != expected {
        return Err(SegmentedCodecError::InvalidChunk);
    }
    Ok(())
}
