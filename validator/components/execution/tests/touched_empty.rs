// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/install_contract.rs"]
mod contract_fixture;
mod support;

use alloy_primitives::{Address, Bytes, KECCAK256_EMPTY, TxKind, U256};
use eve_evm::{compute_state_root, execute_serial_block};
use revm::state::AccountInfo;

#[test]
fn shanghai_clears_touched_precompile_before_next_transaction_extcodehash() {
    let mut parent = support::state();
    let touched = Address::with_last_byte(6);
    let untouched = Address::with_last_byte(0xe8);
    parent.insert_account_info(untouched, AccountInfo::default());
    let reader = Address::with_last_byte(0xcc);
    contract_fixture::install_contract(
        &mut parent,
        reader,
        &hex::decode("60063f60005260206000f3").unwrap(),
    );
    let before = compute_state_root(&parent);
    let call = support::legacy(
        0,
        TxKind::Call(touched),
        U256::ZERO,
        100_000,
        Bytes::from(vec![0; 128]),
    );
    let read = support::legacy(1, TxKind::Call(reader), U256::ZERO, 100_000, Bytes::new());
    let result = execute_serial_block(&parent, &support::environment(), &[call, read]).unwrap();
    assert!(
        result
            .outcomes
            .iter()
            .all(|outcome| outcome.execution.is_success())
    );
    assert_eq!(
        U256::from_be_slice(result.outcomes[1].execution.output().unwrap()),
        U256::ZERO
    );
    assert!(result.state.cache.accounts[&touched].info().is_none());
    assert!(result.state.cache.accounts[&untouched].info().is_some());
    assert_eq!(
        result.state.cache.accounts[&untouched].info.code_hash,
        KECCAK256_EMPTY
    );
    assert_eq!(compute_state_root(&parent), before);
}

#[test]
fn reverted_internal_touch_preserves_existing_empty_account() {
    let mut parent = support::state();
    let empty = Address::with_last_byte(0xe7);
    parent.insert_account_info(empty, AccountInfo::default());
    let caller = Address::with_last_byte(0xcc);
    contract_fixture::install_contract(
        &mut parent,
        caller,
        &hex::decode("6000600060006000600060e762010000f160006000fd").unwrap(),
    );
    let reader = Address::with_last_byte(0xdd);
    contract_fixture::install_contract(
        &mut parent,
        reader,
        &hex::decode("60e73f60005260206000f3").unwrap(),
    );
    let failed = support::legacy(0, TxKind::Call(caller), U256::ZERO, 200_000, Bytes::new());
    let read = support::legacy(1, TxKind::Call(reader), U256::ZERO, 100_000, Bytes::new());
    let result = execute_serial_block(&parent, &support::environment(), &[failed, read]).unwrap();
    assert!(!result.outcomes[0].execution.is_success());
    assert!(!result.outcomes[0].execution.is_halt());
    // EIP-1052 returns zero for both dead-empty and absent accounts. The marker
    // assertion separately proves the reverted touch did not delete this leaf.
    assert_eq!(
        U256::from_be_slice(result.outcomes[1].execution.output().unwrap()),
        U256::ZERO
    );
    assert!(result.state.cache.accounts[&empty].info().is_some());
}

#[test]
fn transaction_boundary_delete_then_value_recreate_cannot_resurrect_old_storage() {
    let mut parent = support::state();
    let empty = Address::with_last_byte(0xe7);
    parent.insert_account_info(empty, AccountInfo::default());
    parent
        .insert_account_storage(empty, U256::from(1), U256::from(7))
        .unwrap();
    // EIP-161 emptiness is nonce/balance/code, independently of a retained slot.
    // Clearing only at block end would see the funded nonempty final account and
    // leave its old slot behind. The first transaction must delete before funding.
    let touch = support::legacy(0, TxKind::Call(empty), U256::ZERO, 21_000, Bytes::new());
    let recreate = support::legacy(1, TxKind::Call(empty), U256::from(1), 21_000, Bytes::new());
    let result =
        execute_serial_block(&parent, &support::environment(), &[touch, recreate]).unwrap();
    assert_eq!(
        result.state.cache.accounts[&empty].info.balance,
        U256::from(1)
    );
    assert!(result.state.cache.accounts[&empty].storage.is_empty());
    assert_eq!(
        parent.cache.accounts[&empty].storage[&U256::from(1)],
        U256::from(7)
    );
    assert_eq!(parent.cache.accounts[&empty].info.balance, U256::ZERO);
}
