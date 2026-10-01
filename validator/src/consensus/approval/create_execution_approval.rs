// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ApprovalError, ExecutionApproval, classify_execution_error};
use crate::consensus::{
    signing::{DurableSigner, check_current_height, signer_status},
    transport::proposals::VerifiedLocalEngineProposal,
};
use alloy_primitives::{B256, Bytes};
use eve_consensus_comet::consensus::certificates::hash_transaction_data;
use eve_evm::{ExecutionBlockInput, execute_state_block};
use eve_state::StateBudget;
use eve_storage::state::read_state_service;

pub(in crate::consensus) fn create_execution_approval(
    signer: &DurableSigner,
    source: &VerifiedLocalEngineProposal,
    budget: &StateBudget,
    reserved_clone_bytes: usize,
) -> Result<ExecutionApproval, ApprovalError> {
    let unavailable = |reason| ApprovalError::Unavailable {
        reason,
        cause: None,
    };
    if signer_status(signer).fenced {
        return Err(unavailable("signer fenced"));
    }
    let request = source.request();
    let binding = source.binding();
    if binding.genesis_hash != signer.config.genesis_hash
        || binding.chain_id != signer.config.chain_id
        || binding.authentication != signer.config.authentication
        || binding.key_epoch != signer.config.key_epoch
    {
        return Err(unavailable("engine proposal signer binding mismatch"));
    }
    check_current_height(signer, request.height)
        .map_err(|_| unavailable("current signer parent unavailable"))?;
    if request.proposer_address.len() != 20
        || (request.height != 1 && binding.previous_consensus_hash == [0; 32])
    {
        return Err(unavailable(
            "invalid native proposer or preceding hash binding",
        ));
    }
    let consensus_hash: [u8; 32] = request
        .hash
        .as_slice()
        .try_into()
        .map_err(|_| unavailable("invalid proposal hash width"))?;
    let time = request
        .time
        .as_ref()
        .ok_or_else(|| unavailable("proposal timestamp missing"))?;
    if time.seconds < 0 || !(0..1_000_000_000).contains(&time.nanos) {
        return Err(unavailable("invalid execution timestamp"));
    }
    let parent = read_state_service(&signer.service)
        .map_err(|_| unavailable("complete state service unavailable"))?;
    if parent
        .commit()
        .target
        .height
        .checked_add(1)
        .and_then(|height| i64::try_from(height).ok())
        != Some(request.height)
    {
        return Err(unavailable(
            "execution parent moved before approval capture",
        ));
    }
    hash_transaction_data(&request.txs).map_err(|_| {
        ApprovalError::InvalidExecution(eve_evm::CompleteExecutionError::Execution(
            eve_evm::BlockExecutionError::InvalidEnvironment("consensus transaction data bounds"),
        ))
    })?;
    let transactions: Vec<Bytes> = request.txs.iter().cloned().map(Bytes::from).collect();
    let input = ExecutionBlockInput {
        timestamp: u64::try_from(time.seconds)
            .map_err(|_| unavailable("invalid proposal timestamp"))?,
        proposer: binding.proposer_owner,
        previous_consensus_hash: B256::from(binding.previous_consensus_hash),
    };
    let prepared = execute_state_block(
        parent.commit(),
        &input,
        &transactions,
        budget,
        reserved_clone_bytes,
    )
    .map_err(classify_execution_error)?;
    if prepared.commit.block.transactions != transactions {
        return Err(unavailable("executed transaction sequence mismatch"));
    }
    let current = read_state_service(&signer.service)
        .map_err(|_| unavailable("complete state service unavailable"))?;
    if current.sequence() != parent.sequence() || current.commit().target != parent.commit().target
    {
        return Err(unavailable("execution parent changed during approval"));
    }
    Ok(ExecutionApproval {
        config: signer.config.clone(),
        parent: parent.commit().target.clone(),
        database_sequence: parent.sequence(),
        consensus_hash,
        request: request.clone(),
        prepared,
    })
}
