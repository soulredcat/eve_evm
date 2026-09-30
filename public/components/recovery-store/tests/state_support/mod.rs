#![allow(dead_code)] // Shared structural storage fixtures used by separate test binaries.
use eve_state::{
    Address, B256, BlockPayload, Bytes, DevelopmentGenesis, GenesisAccount, GenesisValidator,
    SecurityProfile, StateCommit, SystemValue, U256, build_state_commit, compute_evm_root,
    development_economics, development_state_budget, initialize_development_state,
};

pub fn genesis() -> StateCommit {
    let economics = development_economics();
    let keys = [
        "8a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5c",
        "8139770ea87d175f56a35466c34c7ecccb8d8a91b4ee37a25df60f5b8fc9b394",
        "ed4928c628d1c2c6eae90338905995612959273a5c63f93636c14614ac8737d1",
        "ca93ac1705187071d67b83c7ff0efe8108e8ec4530575d7726879333dbdabe7c",
    ];
    let mut accounts: Vec<_> = (1u8..=4)
        .map(|number| GenesisAccount {
            address: Address::repeat_byte(number),
            funded_balance: economics.validator_self_bond * U256::from(10),
            nonce: 0,
            code: Bytes::new(),
        })
        .collect();
    accounts.push(GenesisAccount {
        address: Address::repeat_byte(5),
        funded_balance: U256::from(100),
        nonce: 1,
        code: Bytes::from_static(&[0x60, 0x00, 0x00]),
    });
    let validators = (1u8..=4)
        .map(|number| GenesisValidator {
            owner: Address::repeat_byte(number),
            classical_public_key: keys[usize::from(number - 1)].parse::<B256>().unwrap().0,
            self_bond: economics.validator_self_bond,
            voting_power: 10_000,
        })
        .collect();
    initialize_development_state(
        &DevelopmentGenesis {
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
        },
        &development_state_budget(),
    )
    .unwrap()
}

/// Root-consistent synthetic I/O fixture, not an execution-valid transaction/block claim.
/// Integrated acceptance uses actual execution; these cases target storage failure boundaries.
pub fn next(parent: &StateCommit, revision: u64) -> StateCommit {
    let mut state = parent.state.clone();
    let account = state.accounts.get_mut(&Address::repeat_byte(5)).unwrap();
    account.nonce += 1;
    account.storage.insert(U256::ZERO, U256::from(revision));
    for record in state.system.values_mut() {
        if let SystemValue::Fee { node_pool, .. } = &mut record.value {
            *node_pool = U256::from(revision);
        }
    }
    let mut header = parent.block.header.clone();
    header.parent_hash = parent.target.execution_hash.0;
    header.number = parent.target.height + 1;
    header.timestamp += 1;
    header.state_root = compute_evm_root(&state.accounts).0;
    build_state_commit(
        Some(parent.target.clone()),
        state,
        BlockPayload {
            header,
            transactions: Vec::new(),
            receipts: Vec::new(),
        },
        &development_state_budget(),
    )
    .unwrap()
}
