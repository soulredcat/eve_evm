use crate::support;

use alloy_primitives::{Address, B256, U256};
use eve_protocol_config::{
    native::{NODE_POOL_ADDRESS, SYSTEM_INTERFACE_ADDRESS, VALIDATOR_POOL_ADDRESS},
    network::SecurityProfile,
};
use eve_state::{compute_commit_identity, development_state_budget, initialize_development_state};
use support::genesis::development_genesis;

#[test]
fn tg01_genesis_bond_debits_reserved_escrow_and_supply_match_independent_root() {
    let genesis = development_genesis();
    let initialized = initialize_development_state(&genesis, &development_state_budget()).unwrap();
    let literal: serde_json::Value = serde_json::from_str(include_str!(
        "../fixtures/trie-v1/development_genesis_accounts.json"
    ))
    .unwrap();
    assert_eq!(
        initialized.target.evm_root.0,
        literal["evm_root"]
            .as_str()
            .unwrap()
            .parse::<B256>()
            .unwrap()
    );
    let unit = U256::from(1_000_000_000_000_000_000_u64);
    for value in 1..=4 {
        assert_eq!(
            initialized.state.accounts[&Address::repeat_byte(value)].balance,
            U256::from(90_000) * unit
        );
    }
    assert_eq!(
        initialized.state.accounts[&SYSTEM_INTERFACE_ADDRESS].balance,
        U256::from(40_000) * unit
    );
    assert!(!initialized.state.accounts.contains_key(&NODE_POOL_ADDRESS));
    assert!(
        !initialized
            .state
            .accounts
            .contains_key(&VALIDATOR_POOL_ADDRESS)
    );
    assert_eq!(
        initialized
            .state
            .accounts
            .values()
            .fold(U256::ZERO, |sum, account| sum + account.balance),
        U256::from(400_000) * unit
    );
}

#[test]
fn tg01_input_order_does_not_change_genesis_or_whole_persistent_identity() {
    let first = development_genesis();
    let mut reordered = first.clone();
    reordered.accounts.reverse();
    reordered.validators.reverse();
    let first = initialize_development_state(&first, &development_state_budget()).unwrap();
    let reordered = initialize_development_state(&reordered, &development_state_budget()).unwrap();
    assert_eq!(first, reordered);
    assert_eq!(
        compute_commit_identity(&first, &development_state_budget()).unwrap(),
        compute_commit_identity(&reordered, &development_state_budget()).unwrap()
    );
}

#[test]
fn tg02_invalid_or_production_like_genesis_never_initializes_local_state() {
    for fault in [
        "duplicate",
        "reserved",
        "underfunded",
        "production",
        "hybrid",
    ] {
        let mut genesis = development_genesis();
        match fault {
            "duplicate" => genesis.accounts.push(genesis.accounts[0].clone()),
            "reserved" => genesis.accounts[0].address = SYSTEM_INTERFACE_ADDRESS,
            "underfunded" => genesis.accounts[0].funded_balance = U256::from(1),
            "production" => {
                genesis.network_name = "eve-mainnet".into();
                genesis.evm_chain_id = 1;
            }
            "hybrid" => genesis.profile = SecurityProfile::HybridExperimental,
            _ => unreachable!(),
        }
        assert!(
            initialize_development_state(&genesis, &development_state_budget()).is_err(),
            "{fault}"
        );
    }
}
