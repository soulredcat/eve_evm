// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    SegmentedCodecLimits, SegmentedMarkerMetadata, SegmentedRecordKind, SegmentedSegmentMetadata,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedRecordStats {
    pub kind: SegmentedRecordKind,
    pub encoded_bytes: usize,
    pub header_bytes: usize,
    pub data_bytes: usize,
    pub reference_count: usize,
    /// Requested Vec element storage only, excluding allocator metadata/alignment.
    pub reference_allocation_bytes: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct SegmentedSegmentView<'a> {
    pub metadata: SegmentedSegmentMetadata,
    pub data: &'a [u8],
    pub chunk_hash: [u8; 32],
}

#[derive(Clone, Copy, Debug)]
pub struct SegmentedMarkerView<'a> {
    pub metadata: SegmentedMarkerMetadata,
    pub marker_hash: [u8; 32],
    pub reference_count: usize,
    pub(in crate::records::segmented) references: &'a [u8],
}

#[derive(Debug)]
pub(in crate::records::segmented) enum BorrowedSegmentedRecord<'a> {
    Segment(SegmentedSegmentView<'a>),
    CommitMarker(SegmentedMarkerView<'a>),
}

/// Sealed structural/integrity admission tied to the same immutable raw bytes and
/// frozen limits. This is NOT authenticated finality or a complete-height marker.
#[derive(Debug)]
pub struct SegmentedRecordPreflight<'a> {
    pub(in crate::records::segmented) bytes: &'a [u8],
    pub(in crate::records::segmented) limits: SegmentedCodecLimits,
    pub(in crate::records::segmented) record: BorrowedSegmentedRecord<'a>,
    pub(in crate::records::segmented) stats: SegmentedRecordStats,
}
