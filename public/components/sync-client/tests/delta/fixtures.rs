// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{
    Address, B256, Bytes, DevelopmentGenesis, GenesisAccount, GenesisValidator, SecurityProfile,
    StateVersion, U256, build_state_version, development_economics, development_state_budget,
    initialize_development_state,
};

/// Locally consistent structural version metadata, not a finalized execution history.
pub(super) fn versions() -> (StateVersion, StateVersion) {
    let economics = development_economics();
    let public_keys = [
        "8a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5c",
        "8139770ea87d175f56a35466c34c7ecccb8d8a91b4ee37a25df60f5b8fc9b394",
        "ed4928c628d1c2c6eae90338905995612959273a5c63f93636c14614ac8737d1",
        "ca93ac1705187071d67b83c7ff0efe8108e8ec4530575d7726879333dbdabe7c",
    ];
    let genesis = DevelopmentGenesis {
        schema_version: 1,
        protocol_version: 1,
        network_name: "eve-local-v1".into(),
        evm_chain_id: 31_337,
        initial_timestamp: 1_728_000_000,
        profile: SecurityProfile::ClassicalDev,
        accounts: (1..=4)
            .map(|seed| GenesisAccount {
                address: Address::repeat_byte(seed),
                funded_balance: economics.validator_self_bond * U256::from(10),
                nonce: 0,
                code: Bytes::new(),
            })
            .collect(),
        validators: (1..=4)
            .map(|seed| GenesisValidator {
                owner: Address::repeat_byte(seed),
                classical_public_key: public_keys[usize::from(seed - 1)]
                    .parse::<B256>()
                    .unwrap()
                    .0,
                self_bond: economics.validator_self_bond,
                voting_power: 10_000,
            })
            .collect(),
        economics,
        upgrades: Vec::new(),
    };
    let budget = development_state_budget();
    let commit = initialize_development_state(&genesis, &budget).unwrap();
    let mut header = commit.block.header.clone();
    header.number = 1;
    header.timestamp += 1;
    header.parent_hash = commit.target.execution_hash.0;
    let target = build_state_version(&commit.state, &header, &budget).unwrap();
    (commit.target, target)
}
