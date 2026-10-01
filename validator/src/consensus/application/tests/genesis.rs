// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, Bytes, U256};
use ed25519_dalek::SigningKey;
use eve_state::{
    DevelopmentGenesis, GenesisAccount, GenesisValidator, SecurityProfile, StateCommit,
    development_economics, development_state_budget, initialize_development_state,
};
use std::sync::Arc;

pub(super) fn application_test_genesis(revert: bool) -> (Arc<StateCommit>, DevelopmentGenesis) {
    let economics = development_economics();
    let mut accounts: Vec<_> = (1..=4)
        .map(|index| GenesisAccount {
            address: Address::repeat_byte(index),
            funded_balance: economics.validator_self_bond * U256::from(10),
            nonce: 0,
            code: Bytes::new(),
        })
        .collect();
    accounts.push(GenesisAccount {
        address: "4a62316623ad457f02cdc5d997ded67a383ec569".parse().unwrap(),
        funded_balance: U256::from(10).pow(U256::from(18)),
        nonce: 0,
        code: Bytes::new(),
    });
    accounts.push(GenesisAccount {
        address: Address::with_last_byte(0x42),
        funded_balance: U256::ZERO,
        nonce: 0,
        code: hex::decode(if revert { "60006000fd" } else { "606360005500" })
            .unwrap()
            .into(),
    });
    let validators = (1..=4)
        .map(|index| GenesisValidator {
            owner: Address::repeat_byte(index),
            classical_public_key: SigningKey::from_bytes(&[index; 32])
                .verifying_key()
                .to_bytes(),
            self_bond: economics.validator_self_bond,
            voting_power: 10_000,
        })
        .collect();
    let spec = DevelopmentGenesis {
        schema_version: 1,
        protocol_version: 1,
        network_name: "eve-local-v1".into(),
        evm_chain_id: 31_337,
        initial_timestamp: 1_728_000_000,
        profile: SecurityProfile::ClassicalDev,
        economics,
        accounts,
        validators,
        upgrades: Vec::new(),
    };
    let genesis =
        Arc::new(initialize_development_state(&spec, &development_state_budget()).unwrap());
    (genesis, spec)
}
