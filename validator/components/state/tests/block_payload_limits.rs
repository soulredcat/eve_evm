// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#![cfg(test)]

#[path = "block_payload_codec_support/legacy_block_payload.rs"]
mod legacy_block_payload;
mod support;

use eve_state::{
    Bytes, StateError, decode_block_payload, development_state_budget, encode_block_payload,
};
use legacy_block_payload::encode_legacy_block_payload;

#[test]
fn block_codec_enforces_exact_encoded_size_and_transaction_count_boundaries() {
    let mut block = support::genesis().block;
    let mut budget = development_state_budget();
    let encoded = encode_block_payload(&block, &budget).unwrap();
    budget.maximum_commit_bytes = encoded.len();
    assert_eq!(encode_block_payload(&block, &budget).unwrap(), encoded);
    assert_eq!(decode_block_payload(&encoded, &budget).unwrap(), block);
    budget.maximum_commit_bytes -= 1;
    assert_eq!(
        encode_block_payload(&block, &budget),
        Err(StateError::BudgetExceeded)
    );
    assert_eq!(
        decode_block_payload(&encoded, &budget),
        Err(StateError::BudgetExceeded)
    );
    budget = development_state_budget();
    budget.maximum_journal_operations = 1;
    // Structural wire rows exercise allocation bounds, not execution-valid transactions.
    block.transactions = vec![Bytes::from_static(b"t")];
    block.receipts = vec![Bytes::from_static(b"r")];
    let fitting = encode_block_payload(&block, &budget).unwrap();
    assert_eq!(decode_block_payload(&fitting, &budget).unwrap(), block);
    block.transactions.push(Bytes::from_static(b"u"));
    block.receipts.push(Bytes::from_static(b"s"));
    assert_eq!(
        encode_block_payload(&block, &budget),
        Err(StateError::BudgetExceeded)
    );
    assert_eq!(
        decode_block_payload(&encode_legacy_block_payload(&block), &budget),
        Err(StateError::BudgetExceeded)
    );
}

#[test]
fn block_codec_bounds_each_opaque_transaction_and_receipt_before_copying() {
    let budget = development_state_budget();
    for (transaction_length, receipt_length) in [(131_072, 1), (1, 4_194_304)] {
        let mut block = support::genesis().block;
        block.transactions = vec![Bytes::from(vec![0; transaction_length])];
        block.receipts = vec![Bytes::from(vec![0; receipt_length])];
        let fitting = encode_block_payload(&block, &budget).unwrap();
        assert_eq!(decode_block_payload(&fitting, &budget).unwrap(), block);
        if transaction_length == 131_072 {
            block.transactions[0] = Bytes::from(vec![0; transaction_length + 1]);
        } else {
            block.receipts[0] = Bytes::from(vec![0; receipt_length + 1]);
        }
        assert_eq!(
            encode_block_payload(&block, &budget),
            Err(StateError::BudgetExceeded)
        );
        assert!(decode_block_payload(&encode_legacy_block_payload(&block), &budget).is_err());
    }
}

#[test]
fn block_codec_bounds_aggregate_raw_payload_and_large_header_encoding() {
    let budget = development_state_budget();
    let mut block = support::genesis().block;
    block.transactions = vec![Bytes::from(vec![0; 131_072]); 64];
    block.receipts = vec![Bytes::new(); 64];
    let fitting = encode_block_payload(&block, &budget).unwrap();
    assert_eq!(decode_block_payload(&fitting, &budget).unwrap(), block);
    block.transactions.push(Bytes::from_static(&[1]));
    block.receipts.push(Bytes::new());
    assert_eq!(
        encode_block_payload(&block, &budget),
        Err(StateError::BudgetExceeded)
    );
    assert!(decode_block_payload(&encode_legacy_block_payload(&block), &budget).is_err());
    let mut header_only = support::genesis().block;
    header_only.header.extra_data = Bytes::from(vec![0; 4_096]);
    let mut tight = budget;
    tight.maximum_commit_bytes = 1_024;
    assert_eq!(
        encode_block_payload(&header_only, &tight),
        Err(StateError::BudgetExceeded)
    );
}

#[test]
fn block_codec_rejects_transaction_receipt_count_mismatch() {
    let budget = development_state_budget();
    let mut block = support::genesis().block;
    block.transactions = vec![Bytes::from_static(b"t")];
    assert_eq!(
        encode_block_payload(&block, &budget),
        Err(StateError::CommitMismatch)
    );
    assert_eq!(
        decode_block_payload(&encode_legacy_block_payload(&block), &budget),
        Err(StateError::CommitMismatch)
    );
    block.receipts = vec![Bytes::from_static(b"r"), Bytes::from_static(b"s")];
    assert_eq!(
        encode_block_payload(&block, &budget),
        Err(StateError::CommitMismatch)
    );
    assert!(decode_block_payload(&encode_legacy_block_payload(&block), &budget).is_err());
}
