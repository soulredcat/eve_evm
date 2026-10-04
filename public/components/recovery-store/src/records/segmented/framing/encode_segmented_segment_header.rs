// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{SEGMENTED_SEGMENT_HEADER_BYTES, SegmentedRecordKind, SegmentedSegmentMetadata},
    encode_segmented_base::encode_segmented_base,
};

pub(in crate::records::segmented) fn encode_segmented_segment_header(
    metadata: &SegmentedSegmentMetadata,
    data_length: u32,
) -> [u8; SEGMENTED_SEGMENT_HEADER_BYTES] {
    let mut output = [0; SEGMENTED_SEGMENT_HEADER_BYTES];
    output[..149].copy_from_slice(&encode_segmented_base(
        &metadata.identity,
        SegmentedRecordKind::Segment,
    ));
    output[149..153].copy_from_slice(&metadata.index.to_be_bytes());
    output[153..157].copy_from_slice(&metadata.count.to_be_bytes());
    output[157..165].copy_from_slice(&metadata.offset.to_be_bytes());
    output[165..173].copy_from_slice(&metadata.identity.total_length.to_be_bytes());
    output[173..177].copy_from_slice(&data_length.to_be_bytes());
    output
}
