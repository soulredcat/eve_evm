// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    mempool::{MempoolLimits, start_mempool},
    rpc::{RpcContext, create_rpc_context},
};
use alloy_primitives::{Address, Bytes, U256};
use ed25519_dalek::SigningKey as ValidatorKey;
use eve_protocol_config::{
    genesis::{DevelopmentGenesis, GenesisAccount, GenesisValidator, development_economics},
    network::SecurityProfile,
};
use eve_state::initialize_development_state;
use eve_storage::state::{
    HistoryReadBudget, StateRepository, development_state_storage_budget, ensure_history_index,
    open_state_repository, read_state_service,
};
use std::{path::Path, sync::Arc};
use tempfile::TempDir;

pub(crate) struct RpcFixture {
    pub context: Arc<RpcContext>,
    pub sender: Address,
    pub contract: Address,
    pub repository: StateRepository,
    _directory: TempDir,
}

pub(crate) fn fixture(code: Bytes) -> RpcFixture {
    let economics = development_economics();
    let sender = Address::repeat_byte(5);
    let contract = Address::with_last_byte(0x42);
    let mut accounts: Vec<_> = (1..=5_u8)
        .map(|index| GenesisAccount {
            address: Address::repeat_byte(index),
            nonce: 0,
            code: Bytes::new(),
            funded_balance: U256::from(100_000) * U256::from(10_u64.pow(18)),
        })
        .collect();
    accounts.push(GenesisAccount {
        address: contract,
        nonce: 1,
        code,
        funded_balance: U256::ZERO,
    });
    let validators = (1..=4_u8)
        .map(|index| {
            let entropy =
                k256::ecdsa::SigningKey::random(&mut k256::elliptic_curve::rand_core::OsRng);
            let bytes: [u8; 32] = entropy.to_bytes().into();
            let public = ValidatorKey::from_bytes(&bytes).verifying_key().to_bytes();
            GenesisValidator {
                owner: Address::repeat_byte(index),
                classical_public_key: public,
                self_bond: economics.validator_self_bond,
                voting_power: 10_000,
            }
        })
        .collect();
    let specification = DevelopmentGenesis {
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
    let budget = development_state_storage_budget();
    let genesis = initialize_development_state(&specification, &budget.logical).unwrap();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap();
    let local = root.join("local-tests/b2-rpc-budget");
    std::fs::create_dir_all(&local).unwrap();
    assert!(local.canonicalize().unwrap().starts_with(root));
    let directory = tempfile::tempdir_in(local).unwrap();
    let mut repository = open_state_repository(directory.path(), &genesis, budget).unwrap();
    let history_budget = HistoryReadBudget {
        maximum_block_bytes: 16 * 1_048_576,
        maximum_rebuild_blocks: 128,
        maximum_index_batch_bytes: 16 * 1_048_576,
    };
    assert!(
        ensure_history_index(&mut repository, history_budget)
            .unwrap()
            .complete
    );
    let pool = start_mempool(Arc::new(genesis), MempoolLimits::default());
    let context = create_rpc_context(
        &repository,
        pool,
        budget.logical,
        eve_node_policy::ZoneId(1),
    );
    read_state_service(&context.service).unwrap();
    RpcFixture {
        context,
        sender,
        contract,
        repository,
        _directory: directory,
    }
}
