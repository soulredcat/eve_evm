// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::native_support::{bytes, load_fixture_cases};
use eve_consensus_comet::consensus::certificates::{CertificateError, hash_transaction_data};

#[test]
fn native_data_hash_matches_independent_go_transaction_id_trees() {
    let cases = load_fixture_cases("data", "data_cases");
    assert_eq!(cases.len(), 5);
    for case in cases {
        let transactions: Vec<_> = case["transactions"]
            .as_array()
            .unwrap()
            .iter()
            .map(bytes)
            .collect();
        assert_eq!(
            hash_transaction_data(&transactions).unwrap().as_slice(),
            bytes(&case["hash"]),
            "{}",
            case["name"]
        );
    }
}

#[test]
fn native_data_hash_rejects_dev_raw_transaction_resource_overflow() {
    let oversized = vec![vec![0_u8; 131_073]];
    assert_eq!(
        hash_transaction_data(&oversized),
        Err(CertificateError::InvalidTransactionData)
    );
    let excessive_count = vec![Vec::<u8>::new(); 30_000_000 / 21_000 + 1];
    assert_eq!(
        hash_transaction_data(&excessive_count),
        Err(CertificateError::InvalidTransactionData)
    );
    let excessive_bytes = vec![vec![0_u8; 131_072]; 33];
    assert_eq!(
        hash_transaction_data(&excessive_bytes),
        Err(CertificateError::InvalidTransactionData)
    );
}
