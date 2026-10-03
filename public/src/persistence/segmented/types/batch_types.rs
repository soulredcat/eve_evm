// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{MetadataLease, PARTS, PartLease, SEGMENTS};
use eve_storage::records::segmented::{SegmentedCodecLimits, SegmentedMarkerMetadata};
use eve_storage::records::{OpaqueRecordCursor, OpaqueRecordIdentity};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct SegmentedBatchPlan {
    pub(in crate::persistence::segmented) namespace: OpaqueRecordIdentity,
    pub(in crate::persistence::segmented) codec: SegmentedCodecLimits,
    pub(in crate::persistence::segmented) per_part: usize,
    pub(in crate::persistence::segmented) marker: SegmentedMarkerMetadata,
    pub(in crate::persistence::segmented) segment_count: usize,
    pub(in crate::persistence::segmented) part_count: usize,
    pub(in crate::persistence::segmented) segment_lengths: [usize; SEGMENTS],
    pub(in crate::persistence::segmented) part_lengths: [[usize; 2]; PARTS],
    pub(in crate::persistence::segmented) total_encoded: usize,
}

pub(in crate::persistence::segmented) struct AllocatedPart {
    pub(in crate::persistence::segmented) buffers: [Vec<u8>; 2],
    pub(in crate::persistence::segmented) _lease: PartLease,
}
pub struct SegmentedBatchReservation {
    pub(in crate::persistence::segmented) plan: SegmentedBatchPlan,
    pub(in crate::persistence::segmented) id: u64,
    pub(in crate::persistence::segmented) parts: [Option<AllocatedPart>; PARTS],
    pub(in crate::persistence::segmented) metadata: MetadataLease,
}

pub(in crate::persistence::segmented) struct SealedBatch {
    pub(in crate::persistence::segmented) plan: SegmentedBatchPlan,
    pub(in crate::persistence::segmented) id: u64,
    pub(in crate::persistence::segmented) parts: [Option<AllocatedPart>; PARTS],
    pub(in crate::persistence::segmented) _metadata: MetadataLease,
    pub(in crate::persistence::segmented) expected: OpaqueRecordCursor,
    pub(in crate::persistence::segmented) marker_cursor: OpaqueRecordCursor,
    pub(in crate::persistence::segmented) references: [OpaqueRecordCursor; SEGMENTS],
}
#[derive(Clone)]
pub struct SealedSegmentedBatch(pub(in crate::persistence::segmented) Arc<SealedBatch>);
