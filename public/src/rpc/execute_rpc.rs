// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    RpcContext,
    encoding::{parse_fixed, quantity, require_arity},
    errors::rpc_error,
    run_rpc_worker::run_rpc_worker,
    worker_types::RpcLeases,
};
use alloy_primitives::{Address, B256};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::sync::{Arc, atomic::Ordering};
pub(crate) async fn execute_rpc(
    context: Arc<RpcContext>,
    method: &'static str,
    params: Vec<Value>,
) -> Result<Value, ErrorObjectOwned> {
    if !context.healthy.load(Ordering::Acquire) {
        return Err(rpc_error(
            -32000,
            "NODE_FENCED: reconcile durable state before serving",
        ));
    }
    let active = Arc::clone(&context.active)
        .try_acquire_owned()
        .map_err(|_| rpc_error(-32005, "active RPC capacity exceeded"))?;
    if method == "eth_sendRawTransaction" {
        return super::submit_transaction::submit_transaction(context, params, active).await;
    }
    if method == "eth_getTransactionCount"
        && params.get(1).and_then(Value::as_str) == Some("pending")
    {
        require_arity(&params, 2, 2)?;
        let address = Address::from(parse_fixed::<20>(&params[0])?);
        return context
            .pool
            .pending_nonce(address)
            .await
            .map(quantity)
            .map_err(|e| rpc_error(-32005, e.0));
    }
    let pending = if method == "eth_getTransactionByHash" {
        require_arity(&params, 1, 1)?;
        let hash = B256::from(parse_fixed::<32>(&params[0])?);
        context
            .pool
            .find(hash)
            .await
            .map_err(|e| rpc_error(-32005, e.0))?
    } else {
        None
    };
    let workers = if ["eth_call", "eth_estimateGas"].contains(&method) {
        &context.simulations
    } else if method == "eth_getProof" {
        &context.proofs
    } else {
        &context.histories
    };
    let worker = Arc::clone(workers)
        .try_acquire_owned()
        .map_err(|_| rpc_error(-32005, "RPC worker capacity exceeded"))?;
    let mut reservation =
        super::estimate_rpc_reservation::estimate_rpc_reservation(method, &params)?;
    if [
        "eth_getBlockByNumber",
        "eth_getBlockByHash",
        "eth_getTransactionByHash",
        "eth_getTransactionReceipt",
        "eth_getLogs",
        "eth_feeHistory",
    ]
    .contains(&method)
    {
        reservation = reservation
            .checked_add(
                u32::try_from(
                    context
                        .history_budget
                        .maximum_block_bytes
                        .checked_mul(2)
                        .ok_or_else(|| rpc_error(-32005, "history byte accounting overflow"))?
                        .div_ceil(1024),
                )
                .map_err(|_| rpc_error(-32005, "history byte reservation overflow"))?,
            )
            .ok_or_else(|| rpc_error(-32005, "history reservation overflow"))?;
    }
    let bytes = Arc::clone(&context.bytes)
        .try_acquire_many_owned(reservation)
        .map_err(|_| rpc_error(-32005, "RPC byte capacity exceeded"))?;
    let signature = if [
        "eth_getBlockByNumber",
        "eth_getBlockByHash",
        "eth_getTransactionByHash",
        "eth_getTransactionReceipt",
        "eth_feeHistory",
    ]
    .contains(&method)
    {
        Some(
            Arc::clone(&context.signatures)
                .try_acquire_owned()
                .map_err(|_| rpc_error(-32005, "signature read worker capacity exceeded"))?,
        )
    } else {
        None
    };
    let leases = RpcLeases {
        active,
        worker,
        bytes,
        signature,
    };
    let task = tokio::task::spawn_blocking(move || {
        run_rpc_worker(context, method, params, leases, pending)
    });
    tokio::time::timeout(std::time::Duration::from_secs(30), task)
        .await
        .map_err(|_| {
            rpc_error(
                -32005,
                "RPC timed out; worker retains resource leases until exit",
            )
        })?
        .map_err(|_| rpc_error(-32603, "RPC worker failed"))?
}
