// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_state::{
    Address, B256, Bytes, JournalOperation, StateError, decode_state_journal,
    development_state_budget, encode_state_journal, encode_system_record, preflight_state_journal,
};

#[test]
fn operation_and_encoded_byte_limits_reject_the_complete_input() {
    let journal = fixtures::every_operation();
    let mut budget = development_state_budget();
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    budget.maximum_journal_operations = journal.operations.len() - 1;
    assert_eq!(
        preflight_state_journal(&bytes, &budget),
        Err(StateError::BudgetExceeded)
    );
    assert_eq!(
        decode_state_journal(&bytes, &budget),
        Err(StateError::BudgetExceeded)
    );
    budget = development_state_budget();
    budget.maximum_journal_bytes = bytes.len() - 1;
    assert_eq!(
        preflight_state_journal(&bytes, &budget),
        Err(StateError::BudgetExceeded)
    );
}

#[test]
fn conservative_encoder_budget_and_empty_journal_boundaries_are_preserved() {
    let mut budget = development_state_budget();
    let bytes = encode_state_journal(&fixtures::journal(Vec::new()), &budget).unwrap();
    budget.maximum_journal_operations = 0;
    budget.maximum_journal_bytes = 1_024;
    assert_eq!(
        preflight_state_journal(&bytes, &budget)
            .unwrap()
            .operation_count,
        0
    );
    budget.maximum_journal_bytes -= 1;
    assert_eq!(
        preflight_state_journal(&bytes, &budget),
        Err(StateError::BudgetExceeded)
    );
    let bytes = fixtures::raw_journal(&[fixtures::list(&[
        alloy_rlp::encode(5_u8),
        alloy_rlp::encode(Address::ZERO),
    ])]);
    budget.maximum_journal_operations = 1;
    budget.maximum_journal_bytes = 1_087;
    assert_eq!(
        preflight_state_journal(&bytes, &budget),
        Err(StateError::BudgetExceeded)
    );
    budget.maximum_journal_bytes = 1_088;
    assert!(preflight_state_journal(&bytes, &budget).is_ok());
}

#[test]
fn code_payloads_are_bounded_individually_and_in_aggregate_before_copying() {
    let put = JournalOperation::PutCode {
        code_hash: B256::ZERO,
        code: Bytes::from_static(&[1, 2, 3, 4]),
    };
    let mut budget = development_state_budget();
    let bytes = encode_state_journal(&fixtures::journal(vec![put.clone(), put]), &budget).unwrap();
    budget.maximum_code_bytes = 3;
    assert_eq!(
        preflight_state_journal(&bytes, &budget),
        Err(StateError::BudgetExceeded)
    );
    budget.maximum_code_bytes = 4;
    budget.maximum_total_code_bytes = 7;
    assert_eq!(
        preflight_state_journal(&bytes, &budget),
        Err(StateError::BudgetExceeded)
    );
    budget.maximum_total_code_bytes = 8;
    budget.maximum_codes = 0;
    assert_eq!(
        preflight_state_journal(&bytes, &budget).unwrap().code_bytes,
        8
    );
    assert!(decode_state_journal(&bytes, &budget).is_ok());
}

#[test]
fn system_payload_budget_counts_repeated_records_without_final_state_cardinality() {
    let record = fixtures::parameter_record();
    let record_bytes = encode_system_record(&record).unwrap().len();
    let put = fixtures::put_system(record);
    let mut budget = development_state_budget();
    let bytes = encode_state_journal(&fixtures::journal(vec![put.clone(), put]), &budget).unwrap();
    budget.maximum_system_bytes = record_bytes * 2 - 1;
    assert_eq!(
        preflight_state_journal(&bytes, &budget),
        Err(StateError::BudgetExceeded)
    );
    budget.maximum_system_bytes += 1;
    budget.maximum_system_records = 0;
    let counts = preflight_state_journal(&bytes, &budget).unwrap();
    assert_eq!(
        (counts.system_operations, counts.system_encoded_bytes),
        (2, record_bytes * 2)
    );
    assert!(decode_state_journal(&bytes, &budget).is_ok());
}
