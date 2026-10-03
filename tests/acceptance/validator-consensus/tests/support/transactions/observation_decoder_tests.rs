// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    locate_submitted_transaction::locate_submitted_transaction,
    test_support::{build_located_transaction, build_native_block_response},
    validate_observed_execution::validate_observed_execution,
};
use crate::support::native_decoding::decode_native_block;
use alloy_primitives::Bytes;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

#[test]
fn observed_execution_rejects_wrong_height_count_index_and_nonzero_native_code() {
    for (result, expected) in [
        (
            json!({"height": "8", "txs_results": [{"code": 0}]}),
            "B3_SUBMIT_BLOCK_RESULTS_HEIGHT",
        ),
        (json!({"height": "7"}), "B3_SUBMIT_BLOCK_RESULTS_MISSING"),
        (
            json!({"height": "7", "txs_results": Value::Null}),
            "B3_SUBMIT_BLOCK_RESULTS_MISSING",
        ),
        (
            json!({"height": "7", "txs_results": []}),
            "B3_SUBMIT_BLOCK_RESULTS_COUNT",
        ),
        (
            json!({"height": "7", "txs_results": [{"code": 0}, {"code": 0}]}),
            "B3_SUBMIT_BLOCK_RESULTS_COUNT",
        ),
        (
            json!({"height": "7", "txs_results": [{"code": 1}]}),
            "B3_SUBMIT_TX_RESULT_REJECTED",
        ),
    ] {
        assert_eq!(
            validate_observed_execution(&result, &build_located_transaction(7))
                .unwrap_err()
                .to_string(),
            expected
        );
    }
    let mut located = build_located_transaction(7);
    located.index = 1;
    let result = json!({"height": "7", "txs_results": [{"code": 0}]});
    assert_eq!(
        validate_observed_execution(&result, &located)
            .unwrap_err()
            .to_string(),
        "B3_SUBMIT_BLOCK_RESULTS_INDEX"
    );
}

#[test]
fn located_transaction_requires_exact_unique_bytes_and_native_hash() {
    let raw = Bytes::from_static(&[1, 2, 3]);
    let hash = Sha256::digest(&raw).into();
    let block =
        decode_native_block(&build_native_block_response(7, &[vec![9], raw.to_vec()])).unwrap();
    let located = locate_submitted_transaction(block, &raw, &hash)
        .unwrap()
        .unwrap();
    assert_eq!(located.index, 1);
    assert_eq!(located.block.transactions[located.index], raw.to_vec());
    let block = decode_native_block(&build_native_block_response(7, &[vec![9]])).unwrap();
    assert!(
        locate_submitted_transaction(block, &raw, &hash)
            .unwrap()
            .is_none()
    );
    let block = decode_native_block(&build_native_block_response(
        7,
        &[raw.to_vec(), raw.to_vec()],
    ))
    .unwrap();
    assert_eq!(
        locate_submitted_transaction(block, &raw, &hash)
            .err()
            .unwrap()
            .to_string(),
        "B3_SUBMIT_OBSERVATION_DUPLICATE_TX"
    );
    let block = decode_native_block(&build_native_block_response(7, &[raw.to_vec()])).unwrap();
    assert_eq!(
        locate_submitted_transaction(block, &raw, &[0; 32])
            .err()
            .unwrap()
            .to_string(),
        "B3_SUBMIT_HASH_MISMATCH"
    );
}
