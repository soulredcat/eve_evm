// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SEGMENTED_MARKER_HEADER_BYTES, SEGMENTED_MARKER_OVERHEAD_BYTES, SegmentedCodecError,
    SegmentedCodecLimits, SegmentedLogicalIdentity, SegmentedMarkerMetadata, SegmentedMarkerView,
    SegmentedRecordStats,
    framing::{
        read_segmented_array::read_segmented_array,
        read_segmented_reference::read_segmented_reference,
    },
    types::SegmentedBase,
    validation::validate_segmented_references::validate_segmented_references,
};
use crate::records::OpaqueRecordCursor;
use sha2::{Digest, Sha256};

pub(super) fn preflight_segmented_marker<'a>(
    bytes: &'a [u8],
    base: SegmentedBase,
    limits: &SegmentedCodecLimits,
) -> Result<(SegmentedMarkerView<'a>, SegmentedRecordStats), SegmentedCodecError> {
    if bytes.len() < SEGMENTED_MARKER_OVERHEAD_BYTES {
        return Err(SegmentedCodecError::MalformedEncoding);
    }
    let count = usize::try_from(u32::from_be_bytes(read_segmented_array(bytes, 189)?))
        .map_err(|_| SegmentedCodecError::LimitExceeded)?;
    let reference_bytes = count
        .checked_mul(40)
        .ok_or(SegmentedCodecError::LimitExceeded)?;
    if count == 0
        || count > limits.maximum_segments
        || reference_bytes.checked_add(SEGMENTED_MARKER_OVERHEAD_BYTES) != Some(bytes.len())
    {
        return Err(SegmentedCodecError::InvalidReferences);
    }
    let metadata = SegmentedMarkerMetadata {
        identity: SegmentedLogicalIdentity {
            mode: base.mode,
            logical_id: base.logical_id,
            parent: base.parent,
            target_height: base.target_height,
            total_length: u64::from_be_bytes(read_segmented_array(bytes, 181)?),
        },
        target_state_binding: read_segmented_array(bytes, 149)?,
    };
    let body = bytes.len() - 32;
    let references = &bytes[SEGMENTED_MARKER_HEADER_BYTES..body];
    validate_segmented_references(
        &metadata,
        count,
        (0..count).map(|index| read_segmented_reference(references, index)),
        limits,
    )?;
    let marker_hash = read_segmented_array(bytes, body)?;
    let actual: [u8; 32] = Sha256::digest(&bytes[..body]).into();
    if actual != marker_hash {
        return Err(SegmentedCodecError::HashMismatch);
    }
    Ok((
        SegmentedMarkerView {
            metadata,
            marker_hash,
            reference_count: count,
            references,
        },
        SegmentedRecordStats {
            kind: base.kind,
            encoded_bytes: bytes.len(),
            header_bytes: SEGMENTED_MARKER_HEADER_BYTES,
            data_bytes: 0,
            reference_count: count,
            reference_allocation_bytes: count
                .checked_mul(core::mem::size_of::<OpaqueRecordCursor>())
                .ok_or(SegmentedCodecError::LimitExceeded)?,
        },
    ))
}
