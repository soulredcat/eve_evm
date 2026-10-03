// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{
    AppliedError,
    resources::{
        create_estimated_working_pool, estimate_pending_metadata, estimate_replay_charge,
        estimated_clone_ceiling, reserve_estimated_working, split_estimated_working,
    },
};
use eve_state::development_state_budget;

#[test]
fn checked_estimate_rejects_each_overflow_without_allocating() {
    let mut budget = development_state_budget();
    budget.maximum_accounts = usize::MAX;
    assert!(matches!(
        estimate_replay_charge(&budget, 1, 1),
        Err(AppliedError::ArithmeticOverflow)
    ));
    assert!(matches!(
        estimate_replay_charge(&development_state_budget(), usize::MAX, 1),
        Err(AppliedError::ArithmeticOverflow)
    ));
    assert!(matches!(
        estimate_replay_charge(&development_state_budget(), 1, usize::MAX),
        Err(AppliedError::ArithmeticOverflow)
    ));
}

#[test]
fn default_clone_ceiling_exceeds_public_working_budget_without_raising_limits() {
    let canonical = estimated_clone_ceiling(&development_state_budget()).unwrap();
    assert!(
        canonical
            > eve_node_policy::development_public_budget().maximum_working_state_bytes as usize
    );
}

#[test]
fn real_reservation_split_retains_charge_until_both_leases_drop() {
    let pool = create_estimated_working_pool(10).unwrap();
    let lease = reserve_estimated_working(&pool, 10).unwrap();
    assert!(matches!(
        reserve_estimated_working(&pool, 1),
        Err(AppliedError::EstimatedCapacity)
    ));
    let (retained, transient) = split_estimated_working(lease, 4).unwrap();
    drop(transient);
    let other = reserve_estimated_working(&pool, 6).unwrap();
    assert!(matches!(
        reserve_estimated_working(&pool, 1),
        Err(AppliedError::EstimatedCapacity)
    ));
    drop(retained);
    let released = reserve_estimated_working(&pool, 4).unwrap();
    drop((other, released));
    assert!(reserve_estimated_working(&pool, 10).is_ok());
}

#[test]
fn pending_metadata_count_overflow_and_exact_capacity_are_checked_before_allocation() {
    assert!(matches!(
        estimate_pending_metadata(usize::MAX),
        Err(AppliedError::ArithmeticOverflow)
    ));
    let exact = estimate_pending_metadata(2).unwrap();
    assert!(estimate_pending_metadata(3).unwrap() > exact);
    let pool = create_estimated_working_pool(exact as u64).unwrap();
    let held = reserve_estimated_working(&pool, exact).unwrap();
    assert!(matches!(
        reserve_estimated_working(&pool, 1),
        Err(AppliedError::EstimatedCapacity)
    ));
    drop(held);
    assert!(reserve_estimated_working(&pool, estimate_pending_metadata(3).unwrap()).is_err());
    assert!(reserve_estimated_working(&pool, exact).is_ok());
}
