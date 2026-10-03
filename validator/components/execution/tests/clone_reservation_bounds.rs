// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/build_complete_parent.rs"]
mod parent_fixture;
mod support;

use eve_evm::{
    CompleteExecutionError, estimate_clone_reservation, estimate_clone_reservation_ceiling,
};
use eve_state::{StateError, U256, development_state_budget};

#[test]
fn exact_parent_counts_match_canonical_clone_ceiling() {
    let mut parent = parent_fixture::build_complete_parent(None);
    parent
        .state
        .accounts
        .get_mut(&parent_fixture::CONTRACT)
        .unwrap()
        .storage
        .insert(U256::from(1), U256::from(99));
    let mut budget = development_state_budget();
    budget.maximum_accounts = parent.state.accounts.len();
    budget.maximum_storage_slots = parent
        .state
        .accounts
        .values()
        .map(|account| account.storage.len())
        .sum();
    budget.maximum_codes = parent.state.codes.len();
    budget.maximum_total_code_bytes = parent.state.codes.values().map(|code| code.len()).sum();
    budget.maximum_block_hashes = parent.state.block_hashes.len();
    let actual = estimate_clone_reservation(&parent.state).unwrap();
    assert_eq!(actual, estimate_clone_reservation_ceiling(&budget).unwrap());
    budget.maximum_accounts += 1;
    assert_eq!(
        estimate_clone_reservation_ceiling(&budget).unwrap(),
        actual + 1_024
    );
}

#[test]
fn clone_ceiling_rejects_each_overflowing_resource_dimension() {
    for dimension in 0..5 {
        let mut budget = development_state_budget();
        match dimension {
            0 => budget.maximum_accounts = usize::MAX,
            1 => budget.maximum_storage_slots = usize::MAX,
            2 => budget.maximum_total_code_bytes = usize::MAX,
            3 => budget.maximum_codes = usize::MAX,
            4 => budget.maximum_block_hashes = usize::MAX,
            _ => unreachable!(),
        }
        assert_eq!(
            estimate_clone_reservation_ceiling(&budget),
            Err(CompleteExecutionError::State(
                StateError::ArithmeticOverflow
            ))
        );
    }
}

#[test]
fn default_state_clone_ceiling_exceeds_default_public_working_pool() {
    // A default budget is not evidence that worst-case clones fit a 256 MiB pool.
    assert!(
        estimate_clone_reservation_ceiling(&development_state_budget()).unwrap() > 256 * 1_048_576
    );
}
