// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_log_filter, matches_log};
use crate::rpc::{
    RpcContext,
    encoding::{encode_block_logs, require_arity},
    errors::rpc_error,
    selectors::resolve_height,
};
use eve_storage::state::{capture_history_snapshot, lookup_execution_hash, read_history_block};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn read_logs(context: &RpcContext, params: &[Value]) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 1, 1)?;
    let filter = decode_log_filter(&params[0])?;
    let history = capture_history_snapshot(&context.reader, context.history_budget)
        .map_err(|e| rpc_error(-32000, e.to_string()))?;
    let (from, to) = if let Some(hash) = filter.block_hash {
        let height = lookup_execution_hash(&history, hash)
            .map_err(|e| rpc_error(-32000, e.to_string()))?
            .ok_or_else(|| rpc_error(-32001, "UNKNOWN_BLOCK"))?;
        (height, height)
    } else {
        (
            resolve_height(
                &history,
                filter
                    .from
                    .as_ref()
                    .unwrap_or(&Value::String("latest".into())),
            )?,
            resolve_height(
                &history,
                filter
                    .to
                    .as_ref()
                    .unwrap_or(&Value::String("latest".into())),
            )?,
        )
    };
    if from > to || to > history.version().height {
        return Err(rpc_error(-32001, "invalid or unavailable log range"));
    }
    if to - from >= 1000 {
        return Err(rpc_error(
            -32005,
            "LOG_RANGE_LIMIT: request at most1000blocks",
        ));
    }
    let mut logs = Vec::new();
    let mut bytes = 2_usize;
    for height in from..=to {
        let block = read_history_block(&history, height)
            .map_err(|e| rpc_error(-32000, e.to_string()))?
            .ok_or_else(|| rpc_error(-32001, "HISTORY_NOT_READY"))?;
        for log in encode_block_logs(&block).map_err(|e| rpc_error(-32000, e.to_string()))? {
            if matches_log(&filter, &log) {
                bytes = bytes
                    .checked_add(
                        serde_json::to_vec(&log)
                            .map_err(|e| rpc_error(-32603, e.to_string()))?
                            .len()
                            + 1,
                    )
                    .ok_or_else(|| rpc_error(-32005, "log accounting overflow"))?;
                if logs.len() == 10_000 || bytes > 4 * 1_048_576 {
                    return Err(rpc_error(-32005, "LOG_RESULT_LIMIT: narrow range"));
                }
                logs.push(log);
            }
        }
    }
    serde_json::to_value(logs).map_err(|e| rpc_error(-32603, e.to_string()))
}
