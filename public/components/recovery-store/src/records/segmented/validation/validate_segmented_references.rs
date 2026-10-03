// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{SegmentedCodecError, SegmentedCodecLimits, SegmentedMarkerMetadata},
    validate_segmented_codec_limits::validate_segmented_codec_limits,
    validate_segmented_identity::validate_segmented_identity,
};
use crate::records::OpaqueRecordCursor;

/// One common reference-shape check for owned and borrowed codec inputs.
pub(in crate::records::segmented) fn validate_segmented_references(
    metadata: &SegmentedMarkerMetadata,
    count: usize,
    references: impl Iterator<Item = Result<OpaqueRecordCursor, SegmentedCodecError>>,
    limits: &SegmentedCodecLimits,
) -> Result<(), SegmentedCodecError> {
    validate_segmented_codec_limits(limits)?;
    if metadata.target_state_binding == [0; 32]
        || validate_segmented_identity(&metadata.identity, limits)? != count
    {
        return Err(SegmentedCodecError::InvalidReferences);
    }
    let mut previous = None;
    let mut seen = 0_usize;
    for reference in references {
        let reference = reference?;
        seen = seen
            .checked_add(1)
            .ok_or(SegmentedCodecError::InvalidReferences)?;
        if seen > count
            || reference.content_hash == [0; 32]
            || reference.sequence <= metadata.identity.parent.cursor.sequence
            || previous
                .is_some_and(|sequence: u64| sequence.checked_add(1) != Some(reference.sequence))
        {
            return Err(SegmentedCodecError::InvalidReferences);
        }
        previous = Some(reference.sequence);
    }
    if seen != count {
        return Err(SegmentedCodecError::InvalidReferences);
    }
    Ok(())
}
