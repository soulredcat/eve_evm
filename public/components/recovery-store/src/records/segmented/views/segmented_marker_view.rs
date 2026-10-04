// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedMarkerView, SegmentedRecordPreflight, types::BorrowedSegmentedRecord};

pub fn segmented_marker_view<'a>(
    preflight: &SegmentedRecordPreflight<'a>,
) -> Option<SegmentedMarkerView<'a>> {
    match &preflight.record {
        BorrowedSegmentedRecord::CommitMarker(view) => Some(*view),
        _ => None,
    }
}
