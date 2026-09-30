use crate::development::{
    bootstrap::open_development_store::open_development_store,
    cli::types::DevelopmentOptions,
    config::{
        development_harness_budget::development_harness_budget,
        load_development_genesis::load_development_genesis, read_bounded_input::read_bounded_input,
        types::DevelopmentBlockInput, validate_development_mode::validate_development_mode,
    },
};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{B256, Bloom};
use anyhow::{Context, Result, ensure};
use eve_evm::{BlockEnvironment, FeePoolAddresses, execute_complete_state};
use eve_protocol_config::{
    headers::{ExecutionHeaderInput, build_execution_header},
    native::{NODE_POOL_ADDRESS, VALIDATOR_POOL_ADDRESS},
    records::EvmStateRoot,
};
use eve_state::{BlockPayload, StateVersion, build_state_commit};
use eve_storage::state::{commit_state, create_state_service, read_state_service, state_reader};
use std::path::Path;

/// Real execution followed by local synced publication; this does not create validator finality.
pub fn apply_development_block(
    options: &DevelopmentOptions,
    input_path: &Path,
) -> Result<StateVersion> {
    validate_development_mode(&options.mode, options.acknowledge_unsafe_development)?;
    let input: DevelopmentBlockInput =
        serde_json::from_slice(&read_bounded_input(input_path, 1_048_576)?)?;
    ensure!(
        input.transactions.len() <= 1_024,
        "development block transaction count exceeds limit"
    );
    let genesis = load_development_genesis(&options.genesis)?;
    let mut store = open_development_store(
        &options.root,
        &options.data,
        &options.genesis,
        &options.mode,
        options.acknowledge_unsafe_development,
    )?;
    let service = create_state_service(state_reader(&store));
    let view = read_state_service(&service)?;
    let parent = view.commit();
    ensure!(
        input.timestamp >= parent.target.timestamp,
        "development timestamp precedes parent"
    );
    let budget = development_harness_budget();
    let environment = BlockEnvironment {
        chain_id: parent.target.identity.evm_chain_id,
        number: parent
            .target
            .height
            .checked_add(1)
            .context("execution height overflow")?,
        timestamp: input.timestamp,
        gas_limit: parent.block.header.gas_limit,
        base_fee: parent
            .block
            .header
            .base_fee_per_gas
            .context("missing development base fee")?,
        proposer: genesis
            .validators
            .first()
            .context("development validators missing")?
            .owner,
        previous_consensus_hash: B256::ZERO,
        fee_pools: FeePoolAddresses {
            node_pool: NODE_POOL_ADDRESS,
            validator_pool: VALIDATOR_POOL_ADDRESS,
        },
        maximum_transaction_bytes: 131_072,
    };
    let result = execute_complete_state(
        &parent.state,
        &parent.target,
        &environment,
        &input.transactions,
        &budget.logical,
        16 * 1_048_576,
    )
    .map_err(|error| anyhow::anyhow!("development execution rejected: {error:?}"))?;
    let mut bloom = Bloom::default();
    for outcome in &result.execution.outcomes {
        bloom.accrue_logs(outcome.execution.logs());
    }
    let header = build_execution_header(&ExecutionHeaderInput {
        parent: parent.target.execution_hash,
        beneficiary: environment.proposer,
        state_root: EvmStateRoot(result.execution.state_root),
        transaction_root: result.execution.transactions_root,
        receipt_root: result.execution.receipts_root,
        logs_bloom: bloom,
        number: environment.number,
        gas_limit: environment.gas_limit,
        gas_used: result.execution.gas_used,
        timestamp: environment.timestamp,
        parent_timestamp: parent.target.timestamp,
        previous_consensus_hash: environment.previous_consensus_hash,
        base_fee: environment.base_fee,
        protocol_version: parent.target.identity.protocol_version,
        genesis: parent.target.identity.genesis,
    })
    .map_err(|error| anyhow::anyhow!("development header rejected: {error:?}"))?;
    let receipts = result
        .execution
        .receipts
        .iter()
        .map(Encodable2718::encoded_2718)
        .map(Into::into)
        .collect();
    let commit = build_state_commit(
        Some(parent.target.clone()),
        result.state,
        BlockPayload {
            header,
            transactions: result.execution.transactions,
            receipts,
        },
        &budget.logical,
    )
    .map_err(|error| anyhow::anyhow!("development commit rejected: {error:?}"))?;
    let ack = commit_state(&mut store, &commit)?;
    Ok(ack.committed)
}
