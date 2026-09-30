use alloy_primitives::{Address, B256, Bloom, Bytes};
use eve_evm::{
    BlockEnvironment, FeePoolAddresses, estimate_clone_reservation, execute_complete_state,
};
use eve_protocol_config::{
    headers::{ExecutionHeaderInput, build_execution_header},
    records::EvmStateRoot,
};
use eve_state::{BlockPayload, StateCommit, build_state_commit, development_state_budget};
use serde_json::Value;

use super::{
    commits::structural_genesis,
    fixtures::{expected_root, fixture},
};

pub fn execution_fixture() -> Value {
    serde_json::from_str(include_str!("../fixtures/trie-v1/execution.json")).unwrap()
}

pub fn environment(parent: &StateCommit) -> BlockEnvironment {
    BlockEnvironment {
        chain_id: 31_337,
        number: parent.target.height + 1,
        timestamp: parent.target.timestamp + 1,
        gas_limit: 30_000_000,
        base_fee: 1_000_000_000,
        proposer: Address::repeat_byte(0xaa),
        previous_consensus_hash: B256::repeat_byte(0x33),
        fee_pools: FeePoolAddresses {
            node_pool: "000000000000000000000000000000000000f101".parse().unwrap(),
            validator_pool: "000000000000000000000000000000000000f102".parse().unwrap(),
        },
        maximum_transaction_bytes: 128 * 1024,
    }
}

pub fn executed_commit() -> (StateCommit, StateCommit) {
    let parent = structural_genesis("execution_parent");
    let input = execution_fixture();
    let raw: Bytes = input["raw_transaction"].as_str().unwrap().parse().unwrap();
    let budget = development_state_budget();
    let block = environment(&parent);
    let reservation = estimate_clone_reservation(&parent.state).unwrap();
    let outcome = execute_complete_state(
        &parent.state,
        &parent.target,
        &block,
        &[raw],
        &budget,
        reservation,
    )
    .unwrap();
    let receipt: Bytes = input["raw_receipt"].as_str().unwrap().parse().unwrap();
    assert_eq!(outcome.execution.gas_used, 26_006);
    assert_eq!(outcome.execution.receipts.len(), 1);
    assert_eq!(
        outcome.execution.receipts[0].encoded_2718(),
        receipt.to_vec()
    );
    let post = fixture("execution_post");
    let header = build_execution_header(&ExecutionHeaderInput {
        parent: parent.target.execution_hash,
        beneficiary: block.proposer,
        state_root: EvmStateRoot(expected_root(&post, "evm_root")),
        transaction_root: expected_root(&input, "transaction_root"),
        receipt_root: expected_root(&input, "receipt_root"),
        logs_bloom: Bloom::ZERO,
        number: block.number,
        gas_limit: block.gas_limit,
        gas_used: input["gas_used"].as_str().unwrap().parse().unwrap(),
        timestamp: block.timestamp,
        parent_timestamp: parent.target.timestamp,
        previous_consensus_hash: block.previous_consensus_hash,
        base_fee: block.base_fee,
        protocol_version: parent.target.identity.protocol_version,
        genesis: parent.target.identity.genesis,
    })
    .unwrap();
    let commit = build_state_commit(
        Some(parent.target.clone()),
        outcome.state,
        BlockPayload {
            header,
            transactions: outcome.execution.transactions,
            receipts: vec![receipt],
        },
        &budget,
    )
    .unwrap();
    (parent, commit)
}
use alloy_eips::eip2718::Encodable2718;
