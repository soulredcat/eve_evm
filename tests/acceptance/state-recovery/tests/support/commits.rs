use alloy_consensus::constants::{EMPTY_RECEIPTS, EMPTY_TRANSACTIONS};
use alloy_primitives::{Address, B256, Bloom};
use eve_protocol_config::{
    headers::{ExecutionHeaderInput, build_execution_header},
    records::{EvmStateRoot, ExecutionBlockHash},
};
use eve_state::{
    BlockPayload, StateCommit, build_state_version, development_state_budget, validate_state_commit,
};

use super::fixtures::{complete_state, expected_root, fixture};

/// Structural local fixture, deliberately unrelated to an authenticated genesis.
pub fn structural_genesis(name: &str) -> StateCommit {
    let mut state = complete_state(name);
    let input = fixture(name);
    let header = build_execution_header(&ExecutionHeaderInput {
        parent: ExecutionBlockHash(B256::ZERO),
        beneficiary: Address::repeat_byte(0xaa),
        state_root: EvmStateRoot(expected_root(&input, "evm_root")),
        transaction_root: EMPTY_TRANSACTIONS,
        receipt_root: EMPTY_RECEIPTS,
        logs_bloom: Bloom::ZERO,
        number: 0,
        gas_limit: 30_000_000,
        gas_used: 0,
        timestamp: 1_700_000_000,
        parent_timestamp: 1_700_000_000,
        previous_consensus_hash: B256::ZERO,
        base_fee: 1_000_000_000,
        protocol_version: state.identity.protocol_version,
        genesis: state.identity.genesis,
    })
    .unwrap();
    let budget = development_state_budget();
    let target = build_state_version(&state, &header, &budget).unwrap();
    assert_eq!(target.system_root.0, expected_root(&input, "system_root"));
    state
        .block_hashes
        .insert(target.height, target.execution_hash);
    let commit = StateCommit {
        parent: None,
        target,
        state,
        block: BlockPayload {
            header,
            transactions: Vec::new(),
            receipts: Vec::new(),
        },
    };
    validate_state_commit(&commit, &budget).unwrap();
    commit
}

pub fn empty_child(parent: &StateCommit) -> StateCommit {
    let mut header = parent.block.header.clone();
    header.parent_hash = parent.target.execution_hash.0;
    header.number = parent.target.height + 1;
    header.timestamp += 1;
    header.gas_used = 0;
    header.transactions_root = EMPTY_TRANSACTIONS;
    header.receipts_root = EMPTY_RECEIPTS;
    header.logs_bloom = Bloom::ZERO;
    eve_state::build_state_commit(
        Some(parent.target.clone()),
        parent.state.clone(),
        BlockPayload {
            header,
            transactions: Vec::new(),
            receipts: Vec::new(),
        },
        &development_state_budget(),
    )
    .unwrap()
}
