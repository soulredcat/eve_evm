// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{Bloom, Bytes};
use eve_protocol_config::{
    headers::{ExecutionHeaderInput, build_execution_header, derive_next_base_fee},
    native::{NODE_POOL_ADDRESS, VALIDATOR_POOL_ADDRESS},
    records::EvmStateRoot,
};
use eve_state::{
    BlockPayload, StateBudget, StateCommit, StateError, build_state_commit, project_state_journal,
    validate_state_commit,
};

use super::{ExecutionBlockInput, PreparedStateBlock};
use crate::{BlockEnvironment, CompleteExecutionError, FeePoolAddresses, execute_complete_state};

/// Prepare an atomic complete candidate. The caller owns publication and durability.
pub fn execute_state_block(
    parent: &StateCommit,
    input: &ExecutionBlockInput,
    transactions: &[Bytes],
    budget: &StateBudget,
    reserved_clone_bytes: usize,
) -> Result<PreparedStateBlock, CompleteExecutionError> {
    validate_state_commit(parent, budget).map_err(CompleteExecutionError::State)?;
    let raw_bytes = transactions
        .iter()
        .try_fold(0_usize, |sum, tx| sum.checked_add(tx.len()))
        .ok_or(CompleteExecutionError::State(
            StateError::ArithmeticOverflow,
        ))?;
    // This bounds raw users' bytes. The BFT envelope budget is a separate B3 check.
    let maximum_count = usize::try_from(parent.block.header.gas_limit / 21_000)
        .map_err(|_| CompleteExecutionError::State(StateError::ArithmeticOverflow))?;
    if raw_bytes > 4_194_304 || transactions.len() > maximum_count {
        return Err(CompleteExecutionError::State(StateError::BudgetExceeded));
    }
    let environment = BlockEnvironment {
        chain_id: parent.target.identity.evm_chain_id,
        number: parent
            .target
            .height
            .checked_add(1)
            .ok_or(CompleteExecutionError::State(
                StateError::ArithmeticOverflow,
            ))?,
        timestamp: input.timestamp,
        gas_limit: parent.block.header.gas_limit,
        base_fee: derive_next_base_fee(&parent.block.header)
            .map_err(CompleteExecutionError::Header)?,
        proposer: input.proposer,
        previous_consensus_hash: input.previous_consensus_hash,
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
        transactions,
        budget,
        reserved_clone_bytes,
    )?;
    let mut bloom = Bloom::ZERO;
    for receipt in &result.execution.receipts {
        bloom.accrue_bloom(receipt.logs_bloom());
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
    .map_err(CompleteExecutionError::Header)?;
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
        budget,
    )
    .map_err(CompleteExecutionError::State)?;
    drop(result.journal);
    let journal = project_state_journal(
        &parent.state,
        &parent.target,
        &commit.state,
        commit.target.height,
        budget,
    )
    .map_err(CompleteExecutionError::State)?;
    Ok(PreparedStateBlock {
        commit,
        journal,
        outcomes: result.execution.outcomes,
        fees: result.execution.fees,
    })
}
