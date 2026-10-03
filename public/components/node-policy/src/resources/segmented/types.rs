// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub const SEGMENTED_RECOVERY_POLICY_VERSION: u32 = 2;

/// Numeric local codec/repository contracts, supplied from canonical owners.
/// Required metadata/scratch charges come from actual runtime sizing operations.
/// These knobs authenticate no recovery content and grant no reservation lease.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedRecoveryBounds {
    pub maximum_logical_bytes: u64,
    /// Encoded segment payload including its header/hash, excluding opaque wrapping.
    pub maximum_segment_bytes: u64,
    pub segment_header_bytes: u64,
    pub segment_footer_bytes: u64,
    pub maximum_segments: u64,
    /// Complete encoded marker payload, excluding opaque wrapping.
    pub maximum_marker_bytes: u64,
    pub opaque_record_header_bytes: u64,
    pub maximum_opaque_record_bytes: u64,
    pub maximum_opaque_read_bytes: u64,
    pub maximum_opaque_batch_bytes: u64,
    pub maximum_opaque_batch_records: u64,
    pub required_metadata_bytes: u64,
    pub required_scratch_bytes: u64,
    pub metadata_limit_bytes: u64,
    pub scratch_limit_bytes: u64,
    /// Explicit auxiliary allowance within the unused version 1 process budget.
    pub available_auxiliary_bytes: u64,
}

/// Independent version 2 part-buffer limits. No field renames a version 1 batch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedRecoveryPolicy {
    pub version: u32,
    pub retained_parts: u64,
    /// Aggregate encoded segment/marker payload Vec bytes retained in one part.
    pub maximum_part_bytes: u64,
    pub maximum_segments_per_part: u64,
    pub maximum_segment_bytes: u64,
    pub maximum_marker_bytes: u64,
    pub maximum_metadata_bytes: u64,
    pub maximum_scratch_bytes: u64,
    pub queue_bytes: u64,
    pub maximum_queue_age_ms: u64,
    /// Applied logical heights ahead of durable complete markers, not segment count.
    pub maximum_logical_lag_blocks: u64,
}

/// Worst declared payload/count requirements; neither allocator/RSS nor durability proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedRecoveryCapacity {
    pub data_bytes_per_segment: u64,
    pub maximum_data_segments: u64,
    pub maximum_data_parts: u64,
    pub maximum_retained_parts: u64,
    pub maximum_retained_payload_bytes: u64,
    pub maximum_part_payload_bytes: u64,
    pub maximum_physical_record_bytes: u64,
    pub maximum_marker_record_bytes: u64,
    pub maximum_physical_records: u64,
    pub physical_records_per_transaction: u64,
    pub required_metadata_bytes: u64,
    pub required_scratch_bytes: u64,
    pub reserved_auxiliary_bytes: u64,
}
