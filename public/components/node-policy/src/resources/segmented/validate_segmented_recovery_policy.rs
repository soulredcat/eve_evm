// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    SEGMENTED_RECOVERY_POLICY_VERSION, SegmentedRecoveryBounds, SegmentedRecoveryCapacity,
    SegmentedRecoveryPolicy, available_segmented_allowance::available_segmented_allowance,
    derive_segmented_recovery_capacity::derive_segmented_recovery_capacity,
};
use crate::{BudgetError, PublicBudget, validate_public_budget};

/// Pure startup compatibility; atomic count/byte/metadata leases remain runtime work.
/// Parts retain encoded payloads. Each actual database transaction still has one
/// opaque record. Version 1 queue batches and record counts keep their meanings.
pub fn validate_segmented_recovery_policy(
    base: PublicBudget,
    policy: SegmentedRecoveryPolicy,
    bounds: SegmentedRecoveryBounds,
) -> Result<SegmentedRecoveryCapacity, BudgetError> {
    if policy.version != SEGMENTED_RECOVERY_POLICY_VERSION {
        return Err(BudgetError::UnsupportedVersion);
    }
    validate_public_budget(base)?;
    let positive = [
        policy.retained_parts,
        policy.maximum_part_bytes,
        policy.maximum_segments_per_part,
        policy.maximum_segment_bytes,
        policy.maximum_marker_bytes,
        policy.maximum_metadata_bytes,
        policy.maximum_scratch_bytes,
        policy.queue_bytes,
        policy.maximum_queue_age_ms,
        policy.maximum_logical_lag_blocks,
        bounds.maximum_logical_bytes,
        bounds.maximum_segment_bytes,
        bounds.segment_header_bytes,
        bounds.segment_footer_bytes,
        bounds.maximum_segments,
        bounds.maximum_marker_bytes,
        bounds.opaque_record_header_bytes,
        bounds.maximum_opaque_record_bytes,
        bounds.maximum_opaque_read_bytes,
        bounds.maximum_opaque_batch_bytes,
        bounds.maximum_opaque_batch_records,
        bounds.required_metadata_bytes,
        bounds.required_scratch_bytes,
        bounds.metadata_limit_bytes,
        bounds.scratch_limit_bytes,
        bounds.available_auxiliary_bytes,
    ];
    if positive.contains(&0) {
        return Err(BudgetError::InvalidLimit);
    }
    if policy.maximum_segments_per_part > 2
        || base.maximum_batch_records != 1
        || bounds.maximum_opaque_batch_records != 1
        || policy.maximum_part_bytes > base.maximum_record_bytes
        || policy.maximum_part_bytes > base.maximum_batch_bytes
        || policy.queue_bytes > base.queue_bytes
        || policy.maximum_queue_age_ms > base.queue_age_ms
        || policy.maximum_logical_lag_blocks > base.maximum_durable_lag_blocks
        || policy.maximum_segment_bytes > bounds.maximum_segment_bytes
        || policy.maximum_marker_bytes < bounds.maximum_marker_bytes
        || policy.maximum_marker_bytes > policy.maximum_part_bytes
        || policy.maximum_metadata_bytes < bounds.required_metadata_bytes
        || policy.maximum_scratch_bytes < bounds.required_scratch_bytes
        || policy.maximum_metadata_bytes > bounds.metadata_limit_bytes
        || policy.maximum_scratch_bytes > bounds.scratch_limit_bytes
    {
        return Err(BudgetError::InconsistentBound);
    }
    let retained_capacity = policy
        .retained_parts
        .checked_mul(policy.maximum_part_bytes)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let allowance = available_segmented_allowance(base)?;
    let capacity = derive_segmented_recovery_capacity(policy, bounds)?;
    let physical = capacity
        .maximum_physical_record_bytes
        .max(capacity.maximum_marker_record_bytes);
    if retained_capacity > policy.queue_bytes
        || capacity.maximum_data_segments > bounds.maximum_segments
        || capacity.maximum_retained_parts > policy.retained_parts
        || capacity.maximum_retained_payload_bytes > policy.queue_bytes
        || capacity.maximum_part_payload_bytes > policy.maximum_part_bytes
        || physical > bounds.maximum_opaque_record_bytes
        || physical > bounds.maximum_opaque_read_bytes
        || physical > bounds.maximum_opaque_batch_bytes
        || physical > base.maximum_record_bytes
        || physical > base.maximum_batch_bytes
        || bounds.available_auxiliary_bytes > allowance
        || capacity.reserved_auxiliary_bytes > bounds.available_auxiliary_bytes
    {
        return Err(BudgetError::InconsistentBound);
    }
    Ok(capacity)
}
