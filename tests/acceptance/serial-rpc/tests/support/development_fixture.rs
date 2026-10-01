// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::sign_legacy::ephemeral_signer;
use alloy_primitives::{Address, Bytes, U256};
use ed25519_dalek::SigningKey as ValidatorKey;
use eve_protocol_config::{
    genesis::{DevelopmentGenesis, GenesisAccount, GenesisValidator, development_economics},
    network::SecurityProfile,
};
use eve_state::{StateBudget, StateCommit, development_state_budget, initialize_development_state};
use k256::ecdsa::SigningKey;

pub struct DevelopmentFixture {
    pub parent: StateCommit,
    pub budget: StateBudget,
    pub key: SigningKey,
    pub sender: Address,
    pub contract: Address,
    pub secondary_key: SigningKey,
    pub secondary_sender: Address,
}

pub fn development_fixture(code: Bytes) -> DevelopmentFixture {
    let (key, sender) = ephemeral_signer();
    let (secondary_key, secondary_sender) = ephemeral_signer();
    let contract = Address::with_last_byte(0x42);
    let economics = development_economics();
    let mut accounts: Vec<_> = (1..=4_u8)
        .map(|index| GenesisAccount {
            address: Address::repeat_byte(index),
            nonce: 0,
            code: Bytes::new(),
            funded_balance: U256::from(100_000) * U256::from(10_u64.pow(18)),
        })
        .collect();
    accounts.push(GenesisAccount {
        address: sender,
        nonce: 0,
        code: Bytes::new(),
        funded_balance: U256::from(100_000) * U256::from(10_u64.pow(18)),
    });
    accounts.push(GenesisAccount {
        address: contract,
        nonce: 1,
        code,
        funded_balance: U256::ZERO,
    });
    accounts.push(GenesisAccount {
        address: secondary_sender,
        nonce: 0,
        code: Bytes::new(),
        funded_balance: U256::from(100_000) * U256::from(10_u64.pow(18)),
    });
    let validators = (1..=4_u8)
        .map(|index| {
            // Public genesis contains only these random public keys, never private bytes.
            let (entropy, _) = ephemeral_signer();
            let bytes: [u8; 32] = entropy.to_bytes().into();
            let validator = ValidatorKey::from_bytes(&bytes);
            GenesisValidator {
                owner: Address::repeat_byte(index),
                classical_public_key: validator.verifying_key().to_bytes(),
                self_bond: economics.validator_self_bond,
                voting_power: 10_000,
            }
        })
        .collect();
    let genesis = DevelopmentGenesis {
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
    let budget = development_state_budget();
    let parent = initialize_development_state(&genesis, &budget).unwrap();
    DevelopmentFixture {
        parent,
        budget,
        key,
        sender,
        contract,
        secondary_key,
        secondary_sender,
    }
}
