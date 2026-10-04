// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{SEGMENTED_MARKER_HEADER_BYTES, SegmentedMarkerMetadata, SegmentedRecordKind},
    encode_segmented_base::encode_segmented_base,
};

pub(in crate::records::segmented) fn encode_segmented_marker_header(
    metadata: &SegmentedMarkerMetadata,
    reference_count: u32,
) -> [u8; SEGMENTED_MARKER_HEADER_BYTES] {
    let mut output = [0; SEGMENTED_MARKER_HEADER_BYTES];
    output[..149].copy_from_slice(&encode_segmented_base(
        &metadata.identity,
        SegmentedRecordKind::CommitMarker,
    ));
    output[149..181].copy_from_slice(&metadata.target_state_binding);
    output[181..189].copy_from_slice(&metadata.identity.total_length.to_be_bytes());
    output[189..193].copy_from_slice(&reference_count.to_be_bytes());
    output
}
