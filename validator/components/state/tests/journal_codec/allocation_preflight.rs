// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_state::{
    B256, Bytes, JournalOperation, StateError, decode_state_journal, development_state_budget,
    encode_state_journal, encode_system_record, preflight_state_journal,
};

#[test]
fn borrowed_preflight_counts_every_ordered_code_and_system_payload() {
    let record = fixtures::parameter_record();
    let encoded_record_bytes = encode_system_record(&record).unwrap().len();
    let code = JournalOperation::PutCode {
        code_hash: B256::ZERO,
        code: Bytes::from_static(&[1, 2, 3]),
    };
    let system = fixtures::put_system(record);
    let journal = fixtures::journal(vec![code.clone(), system.clone(), code, system]);
    let budget = development_state_budget();
    let bytes = encode_state_journal(&journal, &budget).unwrap();
    let counts = preflight_state_journal(&bytes, &budget).unwrap();
    assert_eq!(counts.operation_count, 4);
    assert_eq!(
        counts.operation_allocation_bytes,
        4 * core::mem::size_of::<JournalOperation>()
    );
    assert_eq!((counts.code_operations, counts.code_bytes), (2, 6));
    assert_eq!(
        (counts.system_operations, counts.system_encoded_bytes),
        (2, 2 * encoded_record_bytes)
    );
    // Each parameter record has schema, namespace, logical key, name and value leaves.
    assert_eq!(counts.system_leaf_count, 10);
    assert_eq!(counts.system_payload_bytes, 2 * (1 + 9 + 3 + 3 + 5));
    assert_eq!(counts.maximum_system_record_bytes, encoded_record_bytes);
    assert_eq!(
        counts.conservative_codec_scratch_bytes,
        encoded_record_bytes * 8 + 16_384
    );
    assert_eq!(decode_state_journal(&bytes, &budget).unwrap(), journal);
}

#[test]
fn malformed_suffix_is_rejected_during_complete_borrowed_preflight() {
    let budget = development_state_budget();
    let code = fixtures::list(&[
        alloy_rlp::encode(6_u8),
        alloy_rlp::encode(B256::ZERO),
        alloy_rlp::encode(vec![0x60; budget.maximum_code_bytes].as_slice()),
    ]);
    let bytes = fixtures::raw_journal(&[code, fixtures::list(&[alloy_rlp::encode(255_u8)])]);
    assert_eq!(
        preflight_state_journal(&bytes, &budget),
        Err(StateError::MalformedEncoding)
    );
    assert_eq!(
        decode_state_journal(&bytes, &budget),
        Err(StateError::MalformedEncoding)
    );
}
