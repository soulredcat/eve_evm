// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{fixtures, support};
use eve_state::{
    JournalDecodePreflight, JournalOperation, StateError, development_state_budget,
    encode_state_journal, estimate_journal_candidate_reservation,
    estimate_journal_wire_candidate_reservation, preflight_state_journal,
};

#[test]
fn invalid_parent_identity_precedes_even_inconsistent_preflight_statistics() {
    let parent = support::with_slots();
    let mut malformed = parent.state.clone();
    malformed.identity.network_name = "x".repeat(65_536);
    let journal = fixtures::journal(&parent, Vec::new());
    let budget = development_state_budget();
    let counts = JournalDecodePreflight::default();
    assert_eq!(
        estimate_journal_wire_candidate_reservation(&malformed, &counts, &budget),
        Err(StateError::InvalidIdentity)
    );
    assert_eq!(
        estimate_journal_candidate_reservation(&malformed, &journal, &budget),
        Err(StateError::InvalidIdentity)
    );
}

#[test]
fn tighter_operator_wire_and_parent_measurement_budgets_reject() {
    let parent = support::with_slots();
    let journal = fixtures::journal(&parent, fixtures::every_operation(&parent));
    let mut budget = development_state_budget();
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    let counts = preflight_state_journal(&bytes, &budget).unwrap();
    budget.maximum_journal_operations = counts.operation_count - 1;
    assert_eq!(
        estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget),
        Err(StateError::BudgetExceeded)
    );
    budget = development_state_budget();
    budget.maximum_journal_bytes = counts.conservative_journal_bytes - 1;
    assert_eq!(
        estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget),
        Err(StateError::BudgetExceeded)
    );
    budget = development_state_budget();
    budget.maximum_state_bytes = 0;
    assert_eq!(
        estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget),
        Err(StateError::BudgetExceeded)
    );
}

#[test]
fn contradictory_statistical_counts_and_growth_bytes_are_rejected() {
    let parent = support::with_slots();
    let budget = development_state_budget();
    let bytes = encode_state_journal(&fixtures::journal(&parent, Vec::new()), &budget).unwrap();
    let valid = preflight_state_journal(&bytes, &budget).unwrap();
    let mut impossible_insertion = valid;
    impossible_insertion.account_operations = 1;
    let mut impossible_growth = valid;
    impossible_growth.conservative_journal_bytes += 1;
    let mut impossible_allocation = valid;
    impossible_allocation.operation_allocation_bytes = 1;
    let mut impossible_payload = valid;
    impossible_payload.code_bytes = 1;
    for counts in [
        impossible_insertion,
        impossible_growth,
        impossible_allocation,
        impossible_payload,
    ] {
        assert_eq!(
            estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget),
            Err(StateError::MalformedEncoding)
        );
    }
}

#[test]
fn synthetic_count_overflow_rejects_with_checked_arithmetic() {
    let parent = support::with_slots();
    let budget = development_state_budget();
    let bytes = encode_state_journal(&fixtures::journal(&parent, Vec::new()), &budget).unwrap();
    let mut counts = preflight_state_journal(&bytes, &budget).unwrap();
    counts.account_operations = usize::MAX;
    counts.storage_operations = 1;
    assert_eq!(
        estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget),
        Err(StateError::ArithmeticOverflow)
    );
    counts.storage_operations = 0;
    counts.operation_count = usize::MAX;
    assert_eq!(
        estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget),
        Err(StateError::ArithmeticOverflow)
    );
}

#[test]
fn final_state_cardinality_caps_do_not_erase_repeated_insertion_charges() {
    let parent = support::with_slots();
    let operations = fixtures::every_operation(&parent);
    let put = operations
        .iter()
        .find(|operation| matches!(operation, JournalOperation::PutAccount { .. }))
        .unwrap()
        .clone();
    let journal = fixtures::journal(&parent, vec![put.clone(), put]);
    let mut budget = development_state_budget();
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    let counts = preflight_state_journal(&bytes, &budget).unwrap();
    budget.maximum_accounts = 0;
    budget.maximum_storage_slots = 0;
    budget.maximum_codes = 0;
    budget.maximum_system_records = 0;
    budget.maximum_block_hashes = 0;
    assert_eq!(
        estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget),
        estimate_journal_candidate_reservation(&parent.state, &journal, &budget)
    );
    assert!(estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget).is_ok());
}
