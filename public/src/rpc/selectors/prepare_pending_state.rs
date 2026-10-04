// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    mempool::types::PoolCommand,
    rpc::{RpcContext, errors::rpc_error},
};
use alloy_primitives::{Address, B256};
use eve_evm::{ExecutionBlockInput, estimate_clone_reservation, execute_state_block};
use eve_state::StateCommit;
use jsonrpsee::types::ErrorObjectOwned;
use std::sync::Arc;
use tokio::sync::oneshot;
/// Capture one actor-owned head and candidate list; isolated candidate never reaches disk.
pub(crate) fn prepare_pending_state(
    context: &RpcContext,
) -> Result<Arc<StateCommit>, ErrorObjectOwned> {
    if matches!(&context.source, crate::rpc::RpcStateSource::Applied { .. }) {
        return Err(rpc_error(
            -32001,
            "NOT_READY: applied pending overlay unavailable",
        ));
    }
    let (sender, receiver) = oneshot::channel();
    context
        .pool
        .commands
        .try_send(PoolCommand::Capture(sender))
        .map_err(|_| rpc_error(-32005, "mempool command capacity exceeded"))?;
    let (head, candidates) = receiver
        .blocking_recv()
        .map_err(|_| rpc_error(-32000, "mempool stopped"))?;
    let candidates = candidates.map_err(|e| rpc_error(-32000, e.0))?;
    let reservation = estimate_clone_reservation(&head.state)
        .map_err(|e| rpc_error(-32000, format!("pending reservation: {e:?}")))?;
    let charge = u32::try_from(reservation.div_ceil(1024))
        .map_err(|_| rpc_error(-32005, "pending reservation overflow"))?;
    let _lease = Arc::clone(&context.bytes)
        .try_acquire_many_owned(charge)
        .map_err(|_| rpc_error(-32005, "pending byte capacity exceeded"))?;
    let input = ExecutionBlockInput {
        timestamp: head
            .target
            .timestamp
            .checked_add(1)
            .ok_or_else(|| rpc_error(-32000, "pending timestamp overflow"))?,
        proposer: Address::ZERO,
        previous_consensus_hash: B256::ZERO,
    };
    let raw = candidates
        .iter()
        .map(|entry| entry.raw.clone())
        .collect::<Vec<_>>();
    execute_state_block(&head, &input, &raw, &context.state_budget, reservation)
        .map(|prepared| Arc::new(prepared.commit))
        .map_err(|e| rpc_error(-32000, format!("pending execution rejected: {e:?}")))
}
