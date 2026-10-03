// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        SEGMENTED_MARKER_OVERHEAD_BYTES, SegmentedCodecError, SegmentedCodecLimits,
        SegmentedCommitMarker, validation::validate_segmented_marker::validate_segmented_marker,
    },
    write_segmented_marker::write_segmented_marker,
};

pub fn encode_segmented_marker(
    marker: &SegmentedCommitMarker,
    limits: &SegmentedCodecLimits,
) -> Result<Vec<u8>, SegmentedCodecError> {
    validate_segmented_marker(&marker.metadata, &marker.references, limits)?;
    let size = marker
        .references
        .len()
        .checked_mul(40)
        .and_then(|bytes| bytes.checked_add(SEGMENTED_MARKER_OVERHEAD_BYTES))
        .ok_or(SegmentedCodecError::LimitExceeded)?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(size)
        .map_err(|_| SegmentedCodecError::AllocationFailed)?;
    write_segmented_marker(marker, &mut output, limits)?;
    Ok(output)
}
