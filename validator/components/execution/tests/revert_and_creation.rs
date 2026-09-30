#[path = "support/install_contract.rs"]
mod contract_fixture;
mod support;

use alloy_primitives::{Bytes, TxKind, U256, address};
use eve_evm::{BlockExecutionError, compute_state_root, execute_serial_block};

#[test]
fn revert_restores_storage_and_value_but_charges_gas_and_nonce() {
    let mut parent = support::state();
    let contract = address!("00000000000000000000000000000000000000cc");
    contract_fixture::install_contract(
        &mut parent,
        contract,
        &hex::decode("602a60005560006000fd").unwrap(),
    );
    let raw = support::legacy(
        0,
        TxKind::Call(contract),
        U256::from(42),
        100_000,
        Bytes::new(),
    );
    let result = execute_serial_block(&parent, &support::environment(), &[raw]).unwrap();
    assert!(!result.outcomes[0].execution.is_success());
    assert!(!result.outcomes[0].execution.is_halt());
    assert_eq!(result.gas_used, 43_112);
    assert_eq!(
        result.state.cache.accounts[&contract].info.balance,
        U256::ZERO
    );
    assert!(
        result.state.cache.accounts[&contract]
            .storage
            .values()
            .all(U256::is_zero)
    );
    assert_eq!(
        result.state.cache.accounts[&support::sender()].info.nonce,
        1
    );
    assert_eq!(result.fees.collected, U256::from(86_224_000_000_000_u64));
}

#[test]
fn out_of_gas_is_included_as_failure_and_consumes_reserved_gas() {
    let mut parent = support::state();
    let contract = address!("00000000000000000000000000000000000000cc");
    contract_fixture::install_contract(
        &mut parent,
        contract,
        &hex::decode("602a60005500").unwrap(),
    );
    let raw = support::legacy(0, TxKind::Call(contract), U256::ZERO, 21_001, Bytes::new());
    let result = execute_serial_block(&parent, &support::environment(), &[raw]).unwrap();
    assert!(result.outcomes[0].execution.is_halt());
    assert_eq!(result.gas_used, 21_001);
    assert_eq!(
        result.state.cache.accounts[&support::sender()].info.nonce,
        1
    );
    assert!(
        result.state.cache.accounts[&contract]
            .storage
            .values()
            .all(U256::is_zero)
    );
}

#[test]
fn creation_persists_exact_runtime_and_subsequent_call_uses_it() {
    let parent = support::state();
    let runtime = hex::decode("602a60005260206000f3").unwrap();
    let mut initcode = hex::decode("600a600c600039600a6000f3").unwrap();
    initcode.extend_from_slice(&runtime);
    let creation = support::legacy(0, TxKind::Create, U256::ZERO, 200_000, initcode.into());
    let contract = support::sender().create(0);
    let call = support::legacy(1, TxKind::Call(contract), U256::ZERO, 100_000, Bytes::new());
    let result = execute_serial_block(&parent, &support::environment(), &[creation, call]).unwrap();
    assert_eq!(
        result.outcomes[0].execution.created_address(),
        Some(contract)
    );
    let code_hash = result.state.cache.accounts[&contract].info.code_hash;
    assert_eq!(
        result.state.cache.contracts[&code_hash].original_bytes(),
        runtime
    );
    let output = result.outcomes[1].execution.output().unwrap();
    assert_eq!(U256::from_be_slice(output), U256::from(42));
    assert_eq!(
        result.state.cache.accounts[&support::sender()].info.nonce,
        2
    );
}

#[test]
fn invalid_later_nonce_rejects_whole_block_without_partial_parent_mutation() {
    let parent = support::state();
    let root = compute_state_root(&parent);
    let recipient = address!("00000000000000000000000000000000000000bb");
    let first = support::legacy(
        0,
        TxKind::Call(recipient),
        U256::from(1),
        21_000,
        Bytes::new(),
    );
    let invalid = support::legacy(
        0,
        TxKind::Call(recipient),
        U256::from(1),
        21_000,
        Bytes::new(),
    );
    assert!(matches!(
        execute_serial_block(&parent, &support::environment(), &[first, invalid]),
        Err(BlockExecutionError::Execution { index: 1, .. })
    ));
    assert_eq!(compute_state_root(&parent), root);
    assert_eq!(parent.cache.accounts[&support::sender()].info.nonce, 0);
    assert!(!parent.cache.accounts.contains_key(&recipient));
}
