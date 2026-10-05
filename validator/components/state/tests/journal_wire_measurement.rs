// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "journal_codec/fixtures.rs"]
mod fixtures;
mod support;
#[path = "journal_codec/system_records.rs"]
mod system_records;

use eve_state::{
    B256, Bytes, JournalOperation, StateError, U256, development_state_budget,
    encode_state_journal, measure_state_journal_bytes, preflight_state_journal,
};

#[test]
fn exact_measurement_matches_empty_and_every_existing_operation_tag() {
    let budget = development_state_budget();
    let journal = fixtures::every_operation();
    let mut journals = journal
        .operations
        .iter()
        .cloned()
        .map(|operation| fixtures::journal(vec![operation]))
        .collect::<Vec<_>>();
    journals.push(journal);
    journals.push(fixtures::journal(Vec::new()));
    for journal in journals {
        assert_eq!(
            measure_state_journal_bytes(&journal, &budget).unwrap(),
            encode_state_journal(&journal, &budget).unwrap().len()
        );
    }
    assert_eq!(
        measure_state_journal_bytes(&fixtures::journal(Vec::new()), &budget).unwrap(),
        fixtures::raw_journal(&[]).len()
    );
}

#[test]
fn exact_measurement_matches_every_system_namespace_and_optional_parent_commitment() {
    let budget = development_state_budget();
    for record in fixtures::system_records() {
        let journal = fixtures::journal(vec![fixtures::put_system(record)]);
        assert_eq!(
            measure_state_journal_bytes(&journal, &budget).unwrap(),
            encode_state_journal(&journal, &budget).unwrap().len()
        );
    }
    let mut journal = fixtures::every_operation();
    journal.parent = support::with_slots().target;
    journal.target_height = 2;
    assert!(journal.parent.application.is_some());
    assert_eq!(
        measure_state_journal_bytes(&journal, &budget).unwrap(),
        encode_state_journal(&journal, &budget).unwrap().len()
    );
}

#[test]
fn canonical_integer_blob_and_nested_list_boundaries_have_exact_lengths() {
    let budget = development_state_budget();
    for length in [0, 1, 55, 56, 127, 128, 255, 256, 1_023, 1_024, 24_576] {
        for byte in [0_u8, 0x7f, 0x80, 0xff] {
            let journal = fixtures::journal(vec![JournalOperation::PutCode {
                code_hash: B256::ZERO,
                code: Bytes::from(vec![byte; length]),
            }]);
            assert_eq!(
                measure_state_journal_bytes(&journal, &budget).unwrap(),
                encode_state_journal(&journal, &budget).unwrap().len()
            );
        }
    }
    for nonce in [0, 1, 127, 128, 255, 256, 65_535, 65_536, u64::MAX] {
        let journal = fixtures::journal(vec![JournalOperation::PutAccount {
            address: support::contract(),
            nonce,
            balance: U256::from(nonce),
            code_hash: B256::ZERO,
        }]);
        assert_eq!(
            measure_state_journal_bytes(&journal, &budget).unwrap(),
            encode_state_journal(&journal, &budget).unwrap().len()
        );
    }
}

#[test]
fn exact_component_size_keeps_existing_conservative_admission_boundary() {
    let journal = fixtures::every_operation();
    let mut budget = development_state_budget();
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    let counts = preflight_state_journal(&bytes, &budget).unwrap();
    assert!(counts.conservative_journal_bytes > bytes.len());
    budget.maximum_journal_bytes = counts.conservative_journal_bytes;
    assert_eq!(
        measure_state_journal_bytes(&journal, &budget),
        Ok(bytes.len())
    );
    assert_eq!(encode_state_journal(&journal, &budget).unwrap(), bytes);
    budget.maximum_journal_bytes -= 1;
    assert_eq!(
        measure_state_journal_bytes(&journal, &budget),
        Err(StateError::BudgetExceeded)
    );
    assert_eq!(
        encode_state_journal(&journal, &budget),
        Err(StateError::BudgetExceeded)
    );
}

#[test]
fn large_operator_caps_do_not_change_exact_lengths_or_wrap_valid_values() {
    let journal = fixtures::every_operation();
    let expected = encode_state_journal(&journal, &development_state_budget())
        .unwrap()
        .len();
    let mut budget = development_state_budget();
    budget.maximum_journal_bytes = usize::MAX;
    budget.maximum_journal_operations = usize::MAX;
    budget.maximum_code_bytes = usize::MAX;
    assert_eq!(measure_state_journal_bytes(&journal, &budget), Ok(expected));
}

#[test]
fn invalid_fields_and_operation_limits_fail_like_the_existing_encoder() {
    let mut journal = fixtures::every_operation();
    let mut budget = development_state_budget();
    budget.maximum_journal_operations = journal.operations.len() - 1;
    assert_eq!(
        measure_state_journal_bytes(&journal, &budget),
        Err(StateError::BudgetExceeded)
    );
    budget = development_state_budget();
    journal.parent.identity.network_name = "x".repeat(65);
    assert_eq!(
        measure_state_journal_bytes(&journal, &budget),
        Err(StateError::BudgetExceeded)
    );
    let mut record = fixtures::parameter_record();
    record.schema_version = 2;
    let journal = fixtures::journal(vec![fixtures::put_system(record)]);
    assert_eq!(
        measure_state_journal_bytes(&journal, &budget),
        encode_state_journal(&journal, &budget).map(|bytes| bytes.len())
    );
}
