// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{TestFixture, test_key};
use crate::consensus::signing::SignerConfig;
use eve_consensus_comet::consensus::authentication::ConsensusAuthenticationRequirement;
use eve_state::{
    Address, Bytes, DevelopmentGenesis, GenesisAccount, GenesisValidator, SecurityProfile, U256,
    development_economics, development_state_budget, initialize_development_state,
};
use eve_storage::state::{
    create_state_service, development_state_storage_budget, open_state_repository, state_reader,
};
use std::{path::Path, sync::Arc};

pub(in crate::consensus) fn create_fixture(root: &Path) -> TestFixture {
    let economics = development_economics();
    let genesis = initialize_development_state(
        &DevelopmentGenesis {
            schema_version: 1,
            protocol_version: 1,
            network_name: "eve-local-v1".into(),
            evm_chain_id: 31_337,
            initial_timestamp: 1_728_000_000,
            profile: SecurityProfile::ClassicalDev,
            accounts: (1..=4)
                .map(|index| GenesisAccount {
                    address: Address::repeat_byte(index),
                    nonce: 0,
                    code: Bytes::new(),
                    funded_balance: economics.validator_self_bond * U256::from(10),
                })
                .chain([
                    GenesisAccount {
                        address: "4E1c6bB3b3e3e95F6310Eb3058560C83580a4c7B".parse().unwrap(),
                        nonce: 0,
                        code: Bytes::new(),
                        funded_balance: U256::from(10).pow(U256::from(21)),
                    },
                    GenesisAccount {
                        address: Address::with_last_byte(0x42),
                        nonce: 1,
                        code: Bytes::from_static(&[0x60, 0, 0x60, 0, 0xfd]),
                        funded_balance: U256::ZERO,
                    },
                ])
                .collect(),
            validators: (1..=4)
                .map(|index| GenesisValidator {
                    owner: Address::repeat_byte(index),
                    classical_public_key: test_key(index).verifying_key().to_bytes(),
                    self_bond: economics.validator_self_bond,
                    voting_power: 10_000,
                })
                .collect(),
            economics,
            upgrades: Vec::new(),
        },
        &development_state_budget(),
    )
    .unwrap();
    let config = SignerConfig {
        genesis_hash: genesis.target.identity.genesis.0.0,
        chain_id: genesis.target.identity.network_name.clone(),
        expected_public_key: test_key(1).verifying_key().to_bytes(),
        authentication: ConsensusAuthenticationRequirement::ClassicalDev,
        key_epoch: genesis.target.identity.key_epoch,
    };
    let store = open_state_repository(
        &root.join("application"),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    let service = Arc::new(create_state_service(state_reader(&store)));
    TestFixture {
        root: root.to_path_buf(),
        store,
        service,
        config,
    }
}
