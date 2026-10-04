// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{SegmentedMarkerMetadata, SegmentedSegmentMetadata};
use crate::records::OpaqueRecordCursor;

/// Untrusted record contents. SHA-256 protects local byte integrity, not finality.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SegmentedSegment {
    pub metadata: SegmentedSegmentMetadata,
    pub data: Vec<u8>,
    pub chunk_hash: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SegmentedCommitMarker {
    pub metadata: SegmentedMarkerMetadata,
    pub references: Vec<OpaqueRecordCursor>,
    pub marker_hash: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SegmentedRecord {
    Segment(SegmentedSegment),
    CommitMarker(SegmentedCommitMarker),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SegmentedRecordKind {
    Segment = 1,
    CommitMarker = 2,
}
