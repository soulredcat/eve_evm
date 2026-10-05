// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{fixtures, support};
use eve_state::{
    JournalOperation, U256, development_state_budget, encode_state_journal, encode_system_record,
    estimate_journal_candidate_reservation, estimate_journal_wire_candidate_reservation,
    preflight_state_journal,
};

#[test]
fn every_operation_and_empty_journals_share_exact_typed_and_wire_charges() {
    let parent = support::with_slots();
    let budget = development_state_budget();
    let operations = fixtures::every_operation(&parent);
    let mut journals = operations
        .iter()
        .cloned()
        .map(|operation| fixtures::journal(&parent, vec![operation]))
        .collect::<Vec<_>>();
    journals.push(fixtures::journal(&parent, operations));
    journals.push(fixtures::journal(&parent, Vec::new()));
    for journal in journals {
        let bytes = encode_state_journal(&journal, &budget).unwrap();
        let counts = preflight_state_journal(&bytes, &budget).unwrap();
        assert_eq!(
            estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget),
            estimate_journal_candidate_reservation(&parent.state, &journal, &budget)
        );
    }
}

#[test]
fn borrowed_counts_bind_each_possible_insertion_in_the_actual_wire_sequence() {
    let parent = support::with_slots();
    let budget = development_state_budget();
    let operations = fixtures::every_operation(&parent);
    let journal = fixtures::journal(
        &parent,
        operations.iter().chain(&operations).cloned().collect(),
    );
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    let counts = preflight_state_journal(&bytes, &budget).unwrap();
    assert_eq!(counts.operation_count, 20);
    assert_eq!(
        (
            counts.account_operations,
            counts.storage_operations,
            counts.execution_hash_operations
        ),
        (2, 2, 2)
    );
    assert_eq!((counts.code_operations, counts.system_operations), (2, 2));
    assert_eq!(
        estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget),
        estimate_journal_candidate_reservation(&parent.state, &journal, &budget)
    );
    let empty_bytes =
        encode_state_journal(&fixtures::journal(&parent, Vec::new()), &budget).unwrap();
    let empty_counts = preflight_state_journal(&empty_bytes, &budget).unwrap();
    assert_ne!(counts, empty_counts);
    assert!(
        estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget).unwrap()
            > estimate_journal_wire_candidate_reservation(&parent.state, &empty_counts, &budget)
                .unwrap()
    );
}

#[test]
fn per_operation_charge_deltas_preserve_the_existing_cost_model() {
    let parent = support::with_slots();
    let budget = development_state_budget();
    let baseline = estimate_journal_candidate_reservation(
        &parent.state,
        &fixtures::journal(&parent, Vec::new()),
        &budget,
    )
    .unwrap();
    for operation in fixtures::every_operation(&parent) {
        let expected_delta = match &operation {
            JournalOperation::PutAccount { .. } => 1_280,
            JournalOperation::PutStorage { .. } => 1_024,
            JournalOperation::PutCode { code, .. } => 768 + 8 * code.len(),
            JournalOperation::PutSystem { record, .. } => {
                1_024 + 4 * encode_system_record(record).unwrap().len()
            }
            JournalOperation::SetExecutionBlockHash { .. } => 640,
            _ => 512,
        };
        let journal = fixtures::journal(&parent, vec![operation]);
        let bytes = encode_state_journal(&journal, &budget).unwrap();
        let counts = preflight_state_journal(&bytes, &budget).unwrap();
        assert_eq!(
            estimate_journal_candidate_reservation(&parent.state, &journal, &budget).unwrap(),
            baseline + expected_delta
        );
        assert_eq!(
            estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget).unwrap(),
            baseline + expected_delta
        );
    }
}

#[test]
fn zero_storage_and_insert_delete_recreate_never_subtract_peak_growth() {
    let parent = support::with_slots();
    let budget = development_state_budget();
    let put = JournalOperation::PutStorage {
        address: support::contract(),
        slot: U256::ZERO,
        value: U256::ZERO,
    };
    let journal = fixtures::journal(
        &parent,
        vec![
            put.clone(),
            JournalOperation::DeleteStorage {
                address: support::contract(),
                slot: U256::ZERO,
            },
            put,
        ],
    );
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    let counts = preflight_state_journal(&bytes, &budget).unwrap();
    assert_eq!(counts.storage_operations, 2);
    let baseline = estimate_journal_candidate_reservation(
        &parent.state,
        &fixtures::journal(&parent, Vec::new()),
        &budget,
    )
    .unwrap();
    assert_eq!(
        estimate_journal_wire_candidate_reservation(&parent.state, &counts, &budget).unwrap(),
        baseline + 1_024 * 2 + 512
    );
}
