// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedRecordPreflight, SegmentedSegmentView, types::BorrowedSegmentedRecord,
};

pub fn segmented_segment_view<'a>(
    preflight: &SegmentedRecordPreflight<'a>,
) -> Option<SegmentedSegmentView<'a>> {
    match &preflight.record {
        BorrowedSegmentedRecord::Segment(view) => Some(*view),
        _ => None,
    }
}
