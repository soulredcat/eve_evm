// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedCodecError, SegmentedLogicalIdentity, SegmentedRecordPreflight,
    types::BorrowedSegmentedRecord,
};

/// Compare to caller-selected expected metadata; this is not a trust factory.
pub fn match_segmented_identity(
    preflight: &SegmentedRecordPreflight<'_>,
    expected: &SegmentedLogicalIdentity,
) -> Result<(), SegmentedCodecError> {
    let actual = match &preflight.record {
        BorrowedSegmentedRecord::Segment(view) => &view.metadata.identity,
        BorrowedSegmentedRecord::CommitMarker(view) => &view.metadata.identity,
    };
    if actual != expected {
        return Err(SegmentedCodecError::InvalidIdentity);
    }
    Ok(())
}
