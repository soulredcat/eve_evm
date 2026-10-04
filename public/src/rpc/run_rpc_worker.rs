// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    RpcContext, dispatch_rpc::dispatch_rpc, encoding::encode_transaction, errors::rpc_error,
    worker_types::RpcLeases,
};
use crate::mempool::PoolEntry;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::sync::Arc;
pub(crate) fn run_rpc_worker(
    context: Arc<RpcContext>,
    method: &str,
    params: Vec<Value>,
    leases: RpcLeases,
    pending: Option<PoolEntry>,
) -> Result<Value, ErrorObjectOwned> {
    let _leases = (leases.active, leases.worker, leases.bytes, leases.signature);
    let applied_history = leases.applied_history;
    let result = if let Some(entry) = pending {
        encode_transaction(
            &entry.raw,
            entry.validated.evm().chain_id.unwrap_or(0),
            None,
            None,
        )
        .map_err(|e| rpc_error(-32000, e.to_string()))?
    } else if let Some(publication) = &applied_history {
        super::history::dispatch_applied_history(publication, method, &params)?
    } else {
        dispatch_rpc(&context, method, &params)?
    };
    if serde_json::to_vec(&result)
        .map_err(|e| rpc_error(-32603, e.to_string()))?
        .len()
        > 4 * 1_048_576
    {
        return Err(rpc_error(-32005, "RPC response byte capacity exceeded"));
    }
    Ok(result)
}
