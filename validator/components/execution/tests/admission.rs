// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;
#[path = "support/build_typed_transaction.rs"]
mod typed_fixture;

use alloy_eips::eip2930::{AccessList, AccessListItem};
use alloy_primitives::{Bytes, KECCAK256_EMPTY, TxKind, U256};
use eve_evm::{TransactionAdmissionError, check_transaction_admission, decode_signed_transaction};
use eve_state::StateAccount;

#[test]
fn admission_uses_shanghai_intrinsic_upfront_and_state_nonce_metadata() {
    let address = support::environment().proposer;
    let account = StateAccount {
        nonce: 2,
        balance: U256::from(10_u64).pow(U256::from(20)),
        code_hash: KECCAK256_EMPTY,
        storage: Default::default(),
    };
    let raw = support::legacy(
        7,
        TxKind::Call(address),
        U256::from(5),
        30_000,
        Bytes::from_static(&[0, 1]),
    );
    let tx = decode_signed_transaction(&raw, 31_337, 131_072).unwrap();
    let metadata =
        check_transaction_admission(&tx, Some(&account), 1_000_000_000, 30_000_000).unwrap();
    assert_eq!((metadata.nonce, metadata.state_nonce), (7, 2));
    assert_eq!(metadata.intrinsic_gas, 21_020);
    assert_eq!(
        metadata.maximum_upfront_cost,
        U256::from(60_000_000_000_005_u64)
    );
    let access_list = AccessList(vec![AccessListItem {
        address,
        storage_keys: vec![Default::default()],
    }]);
    let raw = typed_fixture::build_typed_transaction(1, access_list, 2, address);
    let tx = decode_signed_transaction(&raw, 31_337, 131_072).unwrap();
    assert_eq!(
        check_transaction_admission(&tx, Some(&account), 1_000_000_000, 30_000_000)
            .unwrap()
            .intrinsic_gas,
        25_300
    );
}

#[test]
fn admission_rejects_intrinsic_caps_code_and_upfront_balance_without_execution() {
    let address = support::environment().proposer;
    let raw = support::legacy(0, TxKind::Call(address), U256::ZERO, 20_999, Bytes::new());
    let tx = decode_signed_transaction(&raw, 31_337, 131_072).unwrap();
    assert_eq!(
        check_transaction_admission(&tx, None, 1_000_000_000, 30_000_000),
        Err(TransactionAdmissionError::IntrinsicGas)
    );
    let raw = support::legacy(0, TxKind::Call(address), U256::ZERO, 21_000, Bytes::new());
    let tx = decode_signed_transaction(&raw, 31_337, 131_072).unwrap();
    assert_eq!(
        check_transaction_admission(&tx, None, 1_000_000_000, 30_000_000),
        Err(TransactionAdmissionError::InsufficientBalance)
    );
    assert_eq!(
        check_transaction_admission(&tx, None, 3_000_000_000, 30_000_000),
        Err(TransactionAdmissionError::FeeCaps)
    );
    let account = StateAccount {
        nonce: 0,
        balance: U256::MAX,
        code_hash: alloy_primitives::keccak256(b"code"),
        storage: Default::default(),
    };
    assert_eq!(
        check_transaction_admission(&tx, Some(&account), 1_000_000_000, 30_000_000),
        Err(TransactionAdmissionError::CallerHasCode)
    );
    let raw = support::legacy(0, TxKind::Call(address), U256::MAX, 21_000, Bytes::new());
    let tx = decode_signed_transaction(&raw, 31_337, 131_072).unwrap();
    let account = StateAccount {
        code_hash: KECCAK256_EMPTY,
        ..account
    };
    assert_eq!(
        check_transaction_admission(&tx, Some(&account), 1_000_000_000, 30_000_000),
        Err(TransactionAdmissionError::ArithmeticOverflow)
    );
}
