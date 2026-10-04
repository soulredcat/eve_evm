// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    SEGMENTED_RECOVERY_POLICY_VERSION, SegmentedRecoveryBounds, SegmentedRecoveryPolicy,
    validate_segmented_recovery_policy,
};
use crate::{BudgetError, PublicBudget};

/// Preserve explicit 4 parts/8 MiB/2 segments and the base queue/age/logical-lag caps.
/// Insufficient operator capacity rejects; this factory never enlarges base pools.
pub fn development_segmented_recovery_policy(
    base: PublicBudget,
    bounds: SegmentedRecoveryBounds,
) -> Result<SegmentedRecoveryPolicy, BudgetError> {
    let policy = SegmentedRecoveryPolicy {
        version: SEGMENTED_RECOVERY_POLICY_VERSION,
        retained_parts: 4,
        maximum_part_bytes: 8 * 1_048_576,
        maximum_segments_per_part: 2,
        maximum_segment_bytes: bounds.maximum_segment_bytes,
        maximum_marker_bytes: bounds.maximum_marker_bytes,
        maximum_metadata_bytes: bounds.metadata_limit_bytes,
        maximum_scratch_bytes: bounds.scratch_limit_bytes,
        queue_bytes: base.queue_bytes,
        maximum_queue_age_ms: base.queue_age_ms,
        maximum_logical_lag_blocks: base.maximum_durable_lag_blocks,
    };
    validate_segmented_recovery_policy(base, policy, bounds)?;
    Ok(policy)
}
