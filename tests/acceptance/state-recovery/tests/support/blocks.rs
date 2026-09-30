use alloy_primitives::{Bloom, Bytes};
use eve_protocol_config::{
    headers::{ExecutionHeaderInput, build_execution_header},
    records::EvmStateRoot,
};
use eve_state::BlockPayload;

use super::{
    commits::structural_genesis,
    execution::execution_fixture,
    fixtures::{expected_root, fixture},
};

/// Canonical envelope fixture; this helper grants no execution/finality authority.
pub fn execution_payload() -> BlockPayload {
    let parent = structural_genesis("execution_parent");
    let input = execution_fixture();
    let post = fixture("execution_post");
    let header = build_execution_header(&ExecutionHeaderInput {
        parent: parent.target.execution_hash,
        beneficiary: parent.block.header.beneficiary,
        state_root: EvmStateRoot(expected_root(&post, "evm_root")),
        transaction_root: expected_root(&input, "transaction_root"),
        receipt_root: expected_root(&input, "receipt_root"),
        logs_bloom: Bloom::ZERO,
        number: 1,
        gas_limit: 30_000_000,
        gas_used: 26_006,
        timestamp: parent.target.timestamp + 1,
        parent_timestamp: parent.target.timestamp,
        previous_consensus_hash: parent.block.header.mix_hash,
        base_fee: 1_000_000_000,
        protocol_version: parent.target.identity.protocol_version,
        genesis: parent.target.identity.genesis,
    })
    .unwrap();
    BlockPayload {
        header,
        transactions: vec![
            input["raw_transaction"]
                .as_str()
                .unwrap()
                .parse::<Bytes>()
                .unwrap(),
        ],
        receipts: vec![
            input["raw_receipt"]
                .as_str()
                .unwrap()
                .parse::<Bytes>()
                .unwrap(),
        ],
    }
}
