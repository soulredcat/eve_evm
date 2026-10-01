// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::sign_legacy::{ephemeral_signer, sign_legacy};
use alloy_primitives::{Address, Bytes, U256, keccak256};
use eve_evm::{BlockEnvironment, FeePoolAddresses, compute_state_root, execute_serial_block};
use eve_protocol_config::native::{NODE_POOL_ADDRESS, VALIDATOR_POOL_ADDRESS};
use revm::{
    database::InMemoryDB,
    state::{AccountInfo, Bytecode},
};

#[test]
fn te02_touched_empty_storage_is_cleared_before_value_recreation_in_next_transaction() {
    let (key, sender) = ephemeral_signer();
    let touched = Address::with_last_byte(0x77);
    let untouched = Address::with_last_byte(0x88);
    let first_probe = Address::with_last_byte(0x42);
    let second_probe = Address::with_last_byte(0x43);
    let mut parent = InMemoryDB::default();
    parent.insert_account_info(
        sender,
        AccountInfo {
            balance: U256::from(10_u128.pow(20)),
            ..AccountInfo::default()
        },
    );
    parent.insert_account_info(untouched, AccountInfo::default());
    // EIP-161's empty predicate excludes storage. This deliberate structural
    // prestate makes delayed deletion observable after the next value transfer.
    parent.insert_account_info(touched, AccountInfo::default());
    parent
        .insert_account_storage(touched, U256::ZERO, U256::from(7))
        .unwrap();
    for (contract, target, slot) in [(first_probe, touched, 0_u8), (second_probe, untouched, 1)] {
        // PUSH20(target), EXTCODEHASH, PUSH1(slot), SSTORE, STOP.
        // EIP-1052 returns zero for both an absent and an EIP-161-empty account.
        let mut raw = vec![0x73];
        raw.extend_from_slice(target.as_slice());
        raw.extend_from_slice(&[0x3f, 0x60, slot, 0x55, 0x00]);
        let code = Bytes::from(raw);
        parent.insert_account_info(
            contract,
            AccountInfo::new(U256::ZERO, 1, keccak256(&code), Bytecode::new_raw(code)),
        );
    }
    let block = BlockEnvironment {
        chain_id: 31_337,
        number: 1,
        timestamp: 1_728_000_001,
        gas_limit: 30_000_000,
        base_fee: 1_000_000_000,
        proposer: Address::with_last_byte(0x99),
        previous_consensus_hash: alloy_primitives::B256::ZERO,
        maximum_transaction_bytes: 131_072,
        fee_pools: FeePoolAddresses {
            node_pool: NODE_POOL_ADDRESS,
            validator_pool: VALIDATOR_POOL_ADDRESS,
        },
    };
    let transactions = [
        sign_legacy(&key, 0, touched, 21_000, U256::ZERO),
        sign_legacy(&key, 1, touched, 21_000, U256::from(1)),
        sign_legacy(&key, 2, first_probe, 100_000, U256::ZERO),
        sign_legacy(&key, 3, second_probe, 100_000, U256::ZERO),
    ];
    let prefix = execute_serial_block(&parent, &block, &transactions[..1]).unwrap();
    assert!(
        prefix
            .state
            .cache
            .accounts
            .get(&touched)
            .and_then(|account| account.info())
            .is_none()
    );
    assert!(prefix.state.cache.accounts[&untouched].info().is_some());
    assert!(prefix.state.cache.accounts[&touched].storage.is_empty());
    let result = execute_serial_block(&parent, &block, &transactions).unwrap();
    assert!(
        result
            .outcomes
            .iter()
            .all(|outcome| outcome.execution.is_success())
    );
    assert_eq!(
        result.state.cache.accounts[&first_probe].storage[&U256::ZERO],
        U256::from_be_slice(alloy_trie::KECCAK_EMPTY.as_slice()),
        "The recreated funded code-less account has KECCAK_EMPTY code hash"
    );
    assert_eq!(
        result.state.cache.accounts[&second_probe]
            .storage
            .get(&U256::from(1))
            .copied()
            .unwrap_or_default(),
        U256::ZERO,
        "An EIP-161-empty account also has EXTCODEHASH zero"
    );
    assert_eq!(
        result.state.cache.accounts[&touched]
            .info()
            .unwrap()
            .balance,
        U256::from(1)
    );
    assert!(
        result.state.cache.accounts[&touched].storage.is_empty(),
        "Cleanup postponed to block end would incorrectly retain the old slot once the account receives value"
    );
    let mut incorrect = result.state.clone();
    incorrect
        .insert_account_storage(touched, U256::ZERO, U256::from(7))
        .unwrap();
    assert_ne!(
        compute_state_root(&incorrect),
        result.state_root,
        "The deleted slot must not survive in the canonical commitment"
    );
    assert!(parent.cache.accounts[&untouched].info().is_some());
    assert!(
        result.state.cache.accounts[&untouched].info().is_some(),
        "Read-only EXTCODEHASH must preserve the explicitly present untouched leaf"
    );
    assert_eq!(
        parent.cache.accounts[&touched].storage[&U256::ZERO],
        U256::from(7),
        "Candidate execution must not alter its parent"
    );
}
