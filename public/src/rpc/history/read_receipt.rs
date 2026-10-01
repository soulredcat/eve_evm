// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{
    RpcContext,
    encoding::{encode_receipt, parse_fixed, require_arity},
    errors::rpc_error,
};
use alloy_primitives::B256;
use eve_storage::state::{capture_history_snapshot, lookup_transaction, read_history_block};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn read_receipt(
    context: &RpcContext,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 1, 1)?;
    let hash = B256::from(parse_fixed::<32>(&params[0])?);
    let history = capture_history_snapshot(&context.reader, context.history_budget)
        .map_err(|e| rpc_error(-32000, e.to_string()))?;
    let Some(location) =
        lookup_transaction(&history, hash).map_err(|e| rpc_error(-32000, e.to_string()))?
    else {
        return Ok(Value::Null);
    };
    let block = read_history_block(&history, location.height)
        .map_err(|e| rpc_error(-32000, e.to_string()))?
        .ok_or_else(|| rpc_error(-32000, "receipt block missing"))?;
    encode_receipt(&block, location.transaction_index as usize)
        .map_err(|e| rpc_error(-32000, e.to_string()))
}
