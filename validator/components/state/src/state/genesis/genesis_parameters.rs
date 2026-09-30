use alloy_primitives::Bytes;
use eve_protocol_config::{
    genesis::DevelopmentGenesis,
    records::{SystemNamespace, SystemRecord, SystemValue},
};

pub(crate) fn genesis_parameters(genesis: &DevelopmentGenesis) -> Vec<SystemRecord> {
    let parameters = [
        (
            "protocol_version",
            alloy_rlp::encode(genesis.protocol_version),
        ),
        ("evm_chain_id", alloy_rlp::encode(genesis.evm_chain_id)),
        ("block_gas_limit", alloy_rlp::encode(30_000_000_u64)),
        ("max_block_bytes", alloy_rlp::encode(4_194_304_u64)),
        ("max_transaction_bytes", alloy_rlp::encode(131_072_u64)),
        ("base_fee_floor", alloy_rlp::encode(1_u64)),
        ("burn_bps", alloy_rlp::encode(genesis.economics.burn_bps)),
        (
            "node_pool_bps",
            alloy_rlp::encode(genesis.economics.node_pool_bps),
        ),
        (
            "validator_pool_bps",
            alloy_rlp::encode(genesis.economics.validator_pool_bps),
        ),
        (
            "epoch_blocks",
            alloy_rlp::encode(genesis.economics.epoch_blocks),
        ),
        (
            "maximum_validators",
            alloy_rlp::encode(genesis.economics.maximum_validators),
        ),
        (
            "validator_self_bond",
            alloy_rlp::encode(genesis.economics.validator_self_bond),
        ),
        (
            "node_self_bond",
            alloy_rlp::encode(genesis.economics.node_self_bond),
        ),
        (
            "unbonding_seconds",
            alloy_rlp::encode(genesis.economics.unbonding_seconds),
        ),
        (
            "unbonding_blocks",
            alloy_rlp::encode(genesis.economics.unbonding_blocks),
        ),
        (
            "maximum_tasks_per_block",
            alloy_rlp::encode(genesis.economics.maximum_tasks_per_block),
        ),
        (
            "default_issuance",
            alloy_rlp::encode(genesis.economics.default_issuance),
        ),
    ];
    parameters
        .into_iter()
        .map(|(name, value)| SystemRecord {
            schema_version: 1,
            namespace: SystemNamespace::Parameter,
            logical_key: Bytes::copy_from_slice(name.as_bytes()),
            value: SystemValue::Parameter {
                name: Bytes::copy_from_slice(name.as_bytes()),
                value: Bytes::from(value),
            },
        })
        .collect()
}
