// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{
    RpcContext,
    encoding::{encode_block, parse_fixed, require_arity},
    errors::rpc_error,
    selectors::resolve_height,
};
use alloy_primitives::B256;
use eve_storage::state::{capture_history_snapshot, lookup_execution_hash, read_history_block};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn read_block(
    context: &RpcContext,
    params: &[Value],
    by_hash: bool,
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 2, 2)?;
    let full = params[1]
        .as_bool()
        .ok_or_else(|| rpc_error(-32602, "full transactions must be boolean"))?;
    if !by_hash && params[0].as_str() == Some("pending") {
        let pending = crate::rpc::selectors::prepare_pending_state(context)?;
        let block = eve_storage::state::RetainedBlockProjection {
            version: pending.target.clone(),
            commit_identity: B256::ZERO,
            block: pending.block.clone(),
        };
        let mut value = encode_block(&block, full).map_err(|e| rpc_error(-32000, e.to_string()))?;
        value["hash"] = Value::Null;
        value["number"] = Value::Null;
        value["evePending"] = Value::Bool(true);
        if full && let Some(transactions) = value["transactions"].as_array_mut() {
            for transaction in transactions {
                transaction["blockHash"] = Value::Null;
                transaction["blockNumber"] = Value::Null;
                transaction["transactionIndex"] = Value::Null;
            }
        }
        return Ok(value);
    }
    let (_, reader) = crate::rpc::durable_rpc_source(context)?;
    let history = capture_history_snapshot(reader, context.history_budget)
        .map_err(|e| rpc_error(-32000, e.to_string()))?;
    let height = if by_hash {
        let hash = B256::from(parse_fixed::<32>(&params[0])?);
        lookup_execution_hash(&history, hash).map_err(|e| rpc_error(-32000, e.to_string()))?
    } else {
        Some(resolve_height(&history, &params[0])?)
    };
    let block = height
        .map(|height| read_history_block(&history, height))
        .transpose()
        .map_err(|e| rpc_error(-32000, e.to_string()))?
        .flatten();
    block
        .map(|block| encode_block(&block, full).map_err(|e| rpc_error(-32000, e.to_string())))
        .transpose()
        .map(|value| value.unwrap_or(Value::Null))
}
