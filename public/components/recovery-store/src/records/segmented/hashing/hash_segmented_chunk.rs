// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedCodecError, SegmentedCodecLimits, SegmentedSegmentMetadata,
    framing::encode_segmented_segment_header::encode_segmented_segment_header,
    validation::validate_segmented_segment::validate_segmented_segment,
};
use sha2::{Digest, Sha256};

/// Standard SHA-256 of the canonical header and chunk bytes, including identity,
/// mode, range and parent metadata. Integrity is not authenticated finality.
pub fn hash_segmented_chunk(
    metadata: &SegmentedSegmentMetadata,
    data: &[u8],
    limits: &SegmentedCodecLimits,
) -> Result<[u8; 32], SegmentedCodecError> {
    validate_segmented_segment(metadata, data.len(), limits)?;
    let length = u32::try_from(data.len()).map_err(|_| SegmentedCodecError::LimitExceeded)?;
    let mut hash = Sha256::new();
    hash.update(encode_segmented_segment_header(metadata, length));
    hash.update(data);
    Ok(hash.finalize().into())
}
