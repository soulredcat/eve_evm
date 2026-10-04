// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SEGMENTED_SEGMENT_HEADER_BYTES, SEGMENTED_SEGMENT_OVERHEAD_BYTES, SegmentedCodecError,
    SegmentedCodecLimits, SegmentedLogicalIdentity, SegmentedRecordStats, SegmentedSegmentMetadata,
    SegmentedSegmentView, framing::read_segmented_array::read_segmented_array,
    types::SegmentedBase, validation::validate_segmented_segment::validate_segmented_segment,
};
use sha2::{Digest, Sha256};

pub(super) fn preflight_segmented_segment<'a>(
    bytes: &'a [u8],
    base: SegmentedBase,
    limits: &SegmentedCodecLimits,
) -> Result<(SegmentedSegmentView<'a>, SegmentedRecordStats), SegmentedCodecError> {
    if bytes.len() < SEGMENTED_SEGMENT_OVERHEAD_BYTES {
        return Err(SegmentedCodecError::MalformedEncoding);
    }
    let length = usize::try_from(u32::from_be_bytes(read_segmented_array(bytes, 173)?))
        .map_err(|_| SegmentedCodecError::LimitExceeded)?;
    if length.checked_add(SEGMENTED_SEGMENT_OVERHEAD_BYTES) != Some(bytes.len()) {
        return Err(SegmentedCodecError::MalformedEncoding);
    }
    let metadata = SegmentedSegmentMetadata {
        identity: SegmentedLogicalIdentity {
            mode: base.mode,
            logical_id: base.logical_id,
            parent: base.parent,
            target_height: base.target_height,
            total_length: u64::from_be_bytes(read_segmented_array(bytes, 165)?),
        },
        index: u32::from_be_bytes(read_segmented_array(bytes, 149)?),
        count: u32::from_be_bytes(read_segmented_array(bytes, 153)?),
        offset: u64::from_be_bytes(read_segmented_array(bytes, 157)?),
    };
    validate_segmented_segment(&metadata, length, limits)?;
    let body = bytes.len() - 32;
    let chunk_hash = read_segmented_array(bytes, body)?;
    let actual: [u8; 32] = Sha256::digest(&bytes[..body]).into();
    if actual != chunk_hash {
        return Err(SegmentedCodecError::HashMismatch);
    }
    Ok((
        SegmentedSegmentView {
            metadata,
            data: &bytes[SEGMENTED_SEGMENT_HEADER_BYTES..body],
            chunk_hash,
        },
        SegmentedRecordStats {
            kind: base.kind,
            encoded_bytes: bytes.len(),
            header_bytes: SEGMENTED_SEGMENT_HEADER_BYTES,
            data_bytes: length,
            reference_count: 0,
            reference_allocation_bytes: 0,
        },
    ))
}
