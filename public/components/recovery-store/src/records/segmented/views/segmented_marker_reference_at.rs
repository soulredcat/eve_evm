// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedCodecError, SegmentedMarkerView,
    framing::read_segmented_reference::read_segmented_reference,
};
use crate::records::OpaqueRecordCursor;

pub fn segmented_marker_reference_at(
    view: &SegmentedMarkerView<'_>,
    index: usize,
) -> Result<OpaqueRecordCursor, SegmentedCodecError> {
    if index >= view.reference_count {
        return Err(SegmentedCodecError::InvalidReferences);
    }
    read_segmented_reference(view.references, index)
}
