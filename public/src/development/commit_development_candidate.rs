// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    mempool::PoolEntry,
    rpc::{RpcContext, RpcEvent, subscriptions::build_rpc_event},
};
use alloy_primitives::{Address, B256};
use anyhow::{Context, Result, anyhow, ensure};
use eve_evm::{ExecutionBlockInput, estimate_clone_reservation, execute_state_block};
use eve_state::StateCommit;
use eve_storage::state::{
    StateRepository, capture_history_snapshot, commit_state, read_history_block, read_state_service,
};
use std::sync::Arc;
pub(crate) fn commit_development_candidate(
    mut repository: StateRepository,
    context: Arc<RpcContext>,
    candidates: Vec<PoolEntry>,
) -> Result<(StateRepository, Arc<StateCommit>, Arc<RpcEvent>)> {
    let (service, reader) = crate::rpc::durable_rpc_source(&context)?;
    let active_head = read_state_service(service)?;
    let parent = active_head.commit();
    let reservation = estimate_clone_reservation(&parent.state)
        .map_err(|e| anyhow!("execution reservation: {e:?}"))?;
    let charge = reservation
        .checked_add(128 * 1_048_576)
        .context("execution byte accounting overflow")?
        .div_ceil(1024);
    ensure!(
        charge <= context.producer_bytes.num_permits(),
        "producer resource capacity exceeded"
    );
    let input = ExecutionBlockInput {
        timestamp: parent
            .target
            .timestamp
            .checked_add(1)
            .context("timestamp overflow")?,
        proposer: Address::ZERO,
        previous_consensus_hash: B256::ZERO,
    };
    let raw = candidates
        .iter()
        .map(|entry| entry.raw.clone())
        .collect::<Vec<_>>();
    let prepared = execute_state_block(parent, &input, &raw, &context.state_budget, reservation)
        .map_err(|e| anyhow!("candidate execution rejected by local profile: {e:?}"))?;
    commit_state(&mut repository, &prepared.commit)?;
    let history = capture_history_snapshot(reader, context.history_budget)?;
    let block = read_history_block(&history, prepared.commit.target.height)?
        .context("new durable block missing")?;
    let event = build_rpc_event(&context, &block)?;
    drop(history);
    Ok((repository, Arc::new(prepared.commit), event))
}
