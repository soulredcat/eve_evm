// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{MIB, bounds};
use eve_node_policy::{
    BudgetError, development_public_budget, development_segmented_recovery_policy,
    validate_segmented_recovery_policy,
};

#[test]
fn exact_part_queue_count_and_marker_limits_pass_and_one_less_rejects() {
    let base = development_public_budget();
    let bounds = bounds();
    let policy = development_segmented_recovery_policy(base, bounds).unwrap();
    let mut too_few_parts = policy;
    too_few_parts.retained_parts = 3;
    let mut too_small_part = policy;
    too_small_part.maximum_part_bytes -= 1;
    let mut too_small_queue = policy;
    too_small_queue.queue_bytes -= 1;
    let mut too_small_marker = policy;
    too_small_marker.maximum_marker_bytes -= 1;
    for smaller in [
        too_few_parts,
        too_small_part,
        too_small_queue,
        too_small_marker,
    ] {
        assert_eq!(
            validate_segmented_recovery_policy(base, smaller, bounds),
            Err(BudgetError::InconsistentBound)
        );
    }
    assert!(validate_segmented_recovery_policy(base, policy, bounds).is_ok());
}

#[test]
fn six_exact_data_segments_fit_and_the_next_byte_requires_forbidden_seventh() {
    let base = development_public_budget();
    let mut bounds = bounds();
    bounds.maximum_logical_bytes = 6 * (4 * MIB - 209);
    let policy = development_segmented_recovery_policy(base, bounds).unwrap();
    assert_eq!(
        validate_segmented_recovery_policy(base, policy, bounds)
            .unwrap()
            .maximum_data_segments,
        6
    );
    bounds.maximum_logical_bytes += 1;
    assert_eq!(
        development_segmented_recovery_policy(base, bounds),
        Err(BudgetError::InconsistentBound)
    );
}

#[test]
fn single_required_nonempty_byte_and_exact_physical_record_limits_are_checked() {
    let base = development_public_budget();
    let mut bounds = bounds();
    bounds.maximum_logical_bytes = 1;
    let policy = development_segmented_recovery_policy(base, bounds).unwrap();
    let capacity = validate_segmented_recovery_policy(base, policy, bounds).unwrap();
    assert_eq!(
        (
            capacity.maximum_data_segments,
            capacity.maximum_data_parts,
            capacity.maximum_retained_parts
        ),
        (1, 1, 2)
    );
    bounds.maximum_opaque_record_bytes = capacity.maximum_physical_record_bytes;
    bounds.maximum_opaque_read_bytes = capacity.maximum_physical_record_bytes;
    assert!(validate_segmented_recovery_policy(base, policy, bounds).is_ok());
    bounds.maximum_opaque_read_bytes -= 1;
    assert_eq!(
        validate_segmented_recovery_policy(base, policy, bounds),
        Err(BudgetError::InconsistentBound)
    );
    bounds.maximum_logical_bytes = 0;
    assert_eq!(
        development_segmented_recovery_policy(base, bounds),
        Err(BudgetError::InvalidLimit)
    );
}

#[test]
fn stricter_operator_base_is_rejected_instead_of_automatically_enlarged() {
    let mut base = development_public_budget();
    base.maximum_record_bytes = 8 * MIB - 1;
    assert_eq!(
        development_segmented_recovery_policy(base, bounds()),
        Err(BudgetError::InconsistentBound)
    );
    assert_eq!(base.maximum_record_bytes, 8 * MIB - 1);
    let mut bounds = bounds();
    bounds.scratch_limit_bytes = bounds.required_scratch_bytes - 1;
    assert_eq!(
        development_segmented_recovery_policy(development_public_budget(), bounds),
        Err(BudgetError::InconsistentBound)
    );
    assert_eq!(bounds.scratch_limit_bytes, 33_566_807);
}
