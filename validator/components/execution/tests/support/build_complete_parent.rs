// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{
    Address, B256, Bytes, DevelopmentGenesis, GenesisAccount, GenesisValidator, SecurityProfile,
    StateCommit, U256, development_economics, development_state_budget,
    initialize_development_state,
};

pub const CONTRACT: Address = Address::new([5; 20]);

/// Own-package public fixture data; canonical genesis behavior remains in eve-state.
pub fn build_complete_parent(runtime: Option<&[u8]>) -> StateCommit {
    let economics = development_economics();
    let keys = [
        "8a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5c",
        "8139770ea87d175f56a35466c34c7ecccb8d8a91b4ee37a25df60f5b8fc9b394",
        "ed4928c628d1c2c6eae90338905995612959273a5c63f93636c14614ac8737d1",
        "ca93ac1705187071d67b83c7ff0efe8108e8ec4530575d7726879333dbdabe7c",
    ];
    let mut accounts = (1_u8..=4)
        .map(|number| GenesisAccount {
            address: Address::repeat_byte(number),
            funded_balance: economics.validator_self_bond * U256::from(10),
            nonce: 0,
            code: Bytes::new(),
        })
        .collect::<Vec<_>>();
    accounts.push(GenesisAccount {
        address: crate::support::sender(),
        funded_balance: U256::from(10_u64).pow(U256::from(20)),
        nonce: 0,
        code: Bytes::new(),
    });
    accounts.push(GenesisAccount {
        address: CONTRACT,
        funded_balance: U256::from(100),
        nonce: 1,
        code: Bytes::copy_from_slice(runtime.unwrap_or(&[0x60, 0x63, 0x60, 0, 0x55, 0])),
    });
    let validators = (1_u8..=4)
        .map(|number| GenesisValidator {
            owner: Address::repeat_byte(number),
            classical_public_key: keys[usize::from(number - 1)].parse::<B256>().unwrap().0,
            self_bond: economics.validator_self_bond,
            voting_power: 10_000,
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
    initialize_development_state(&genesis, &development_state_budget()).unwrap()
}
