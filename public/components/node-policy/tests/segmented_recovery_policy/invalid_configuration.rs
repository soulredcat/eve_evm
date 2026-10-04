// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{MIB, bounds};
use eve_node_policy::{
    BudgetError, development_public_budget, development_segmented_recovery_policy,
    validate_segmented_recovery_policy,
};

#[test]
fn unsupported_versions_and_nonempty_required_limits_fail_closed() {
    let base = development_public_budget();
    let bounds = bounds();
    let policy = development_segmented_recovery_policy(base, bounds).unwrap();
    for version in [0, 1, 3, u32::MAX] {
        let mut policy = policy;
        policy.version = version;
        assert_eq!(
            validate_segmented_recovery_policy(base, policy, bounds),
            Err(BudgetError::UnsupportedVersion)
        );
    }
    let mut bad_base = base;
    bad_base.version = 2;
    assert_eq!(
        development_segmented_recovery_policy(bad_base, bounds),
        Err(BudgetError::UnsupportedVersion)
    );
    let mut bad_bounds = bounds;
    bad_bounds.required_metadata_bytes = 0;
    assert_eq!(
        development_segmented_recovery_policy(base, bad_bounds),
        Err(BudgetError::InvalidLimit)
    );
    bad_bounds = bounds;
    bad_bounds.required_scratch_bytes = 0;
    assert_eq!(
        development_segmented_recovery_policy(base, bad_bounds),
        Err(BudgetError::InvalidLimit)
    );
    let mut empty_policy = policy;
    empty_policy.maximum_segments_per_part = 0;
    assert_eq!(
        validate_segmented_recovery_policy(base, empty_policy, bounds),
        Err(BudgetError::InvalidLimit)
    );
}

#[test]
fn impossible_segment_transaction_age_and_lag_configs_are_rejected() {
    let base = development_public_budget();
    let bounds = bounds();
    let policy = development_segmented_recovery_policy(base, bounds).unwrap();
    let mut over_segment_limit = policy;
    over_segment_limit.maximum_segments_per_part = 3;
    let mut only_one_segment = policy;
    only_one_segment.maximum_segments_per_part = 1;
    let mut over_age = policy;
    over_age.maximum_queue_age_ms += 1;
    let mut over_lag = policy;
    over_lag.maximum_logical_lag_blocks += 1;
    for invalid in [over_segment_limit, only_one_segment, over_age, over_lag] {
        assert_eq!(
            validate_segmented_recovery_policy(base, invalid, bounds),
            Err(BudgetError::InconsistentBound)
        );
    }
    let mut physical = bounds;
    physical.maximum_opaque_batch_records = 2;
    assert_eq!(
        development_segmented_recovery_policy(base, physical),
        Err(BudgetError::InconsistentBound)
    );
    let mut wider_base = base;
    wider_base.maximum_batch_records = 2;
    assert_eq!(
        development_segmented_recovery_policy(wider_base, bounds),
        Err(BudgetError::InconsistentBound)
    );
    let mut fictitious_allowance = bounds;
    fictitious_allowance.available_auxiliary_bytes = 60 * MIB + 1;
    assert_eq!(
        development_segmented_recovery_policy(base, fictitious_allowance),
        Err(BudgetError::InconsistentBound)
    );
}

#[test]
fn overflow_in_retained_capacity_headers_and_auxiliary_charges_is_rejected() {
    let base = development_public_budget();
    let bounds = bounds();
    let policy = development_segmented_recovery_policy(base, bounds).unwrap();
    let mut overflowing_parts = policy;
    overflowing_parts.retained_parts = u64::MAX;
    assert_eq!(
        validate_segmented_recovery_policy(base, overflowing_parts, bounds),
        Err(BudgetError::ArithmeticOverflow)
    );
    let mut overflowing_header = bounds;
    overflowing_header.segment_header_bytes = u64::MAX;
    assert_eq!(
        development_segmented_recovery_policy(base, overflowing_header),
        Err(BudgetError::ArithmeticOverflow)
    );
    let mut overflowing_auxiliary = bounds;
    overflowing_auxiliary.metadata_limit_bytes = u64::MAX;
    assert_eq!(
        development_segmented_recovery_policy(base, overflowing_auxiliary),
        Err(BudgetError::ArithmeticOverflow)
    );
}
