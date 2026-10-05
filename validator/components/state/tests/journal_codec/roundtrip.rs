// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_state::{
    Address, B256, Bytes, JournalOperation, StateJournal, U256, decode_state_journal,
    development_state_budget, encode_state_journal, encode_system_record, preflight_state_journal,
};

#[test]
fn every_operation_roundtrips_with_exact_counts_and_unchanged_wire_bytes() {
    let journal = fixtures::every_operation();
    let budget = development_state_budget();
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    let counts = preflight_state_journal(&bytes, &budget).unwrap();
    assert_eq!(counts.encoded_bytes, bytes.len());
    assert_eq!(counts.operation_count, 10);
    assert_eq!(
        counts.operation_allocation_bytes,
        10 * core::mem::size_of::<JournalOperation>()
    );
    assert_eq!((counts.code_operations, counts.code_bytes), (1, 4));
    assert_eq!(counts.system_operations, 1);
    assert_eq!(
        counts.system_encoded_bytes,
        encode_system_record(&fixtures::parameter_record())
            .unwrap()
            .len()
    );
    assert_eq!(
        counts.parent_network_name_bytes,
        journal.parent.identity.network_name.len()
    );
    let decoded = decode_state_journal(&bytes, &budget).unwrap();
    assert_eq!(decoded, journal);
    assert_eq!(encode_state_journal(&decoded, &budget).unwrap(), bytes);
}

#[test]
fn all_system_namespaces_use_the_existing_canonical_record_decoder() {
    let journal = fixtures::journal(
        fixtures::system_records()
            .into_iter()
            .map(fixtures::put_system)
            .collect(),
    );
    let budget = development_state_budget();
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    assert_eq!(
        preflight_state_journal(&bytes, &budget)
            .unwrap()
            .system_operations,
        7
    );
    assert_eq!(decode_state_journal(&bytes, &budget).unwrap(), journal);
}

#[test]
fn repeated_operations_and_delete_recreate_keep_execution_order() {
    let address = Address::repeat_byte(9);
    let put = JournalOperation::PutAccount {
        address,
        nonce: 0,
        balance: U256::from(1),
        code_hash: B256::repeat_byte(1),
    };
    let journal = fixtures::journal(vec![
        put.clone(),
        put.clone(),
        JournalOperation::DeleteAccount { address },
        put,
    ]);
    let budget = development_state_budget();
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    assert_eq!(decode_state_journal(&bytes, &budget).unwrap(), journal);
}

#[test]
fn empty_operations_empty_code_and_optional_parent_commitment_roundtrip() {
    let budget = development_state_budget();
    let mut journal = StateJournal {
        parent: super::support::with_slots().target,
        target_height: 2,
        operations: Vec::new(),
    };
    assert!(journal.parent.application.is_some());
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    assert_eq!(decode_state_journal(&bytes, &budget).unwrap(), journal);
    journal.operations.push(JournalOperation::PutCode {
        code_hash: B256::ZERO,
        code: Bytes::new(),
    });
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    assert_eq!(decode_state_journal(&bytes, &budget).unwrap(), journal);
}
