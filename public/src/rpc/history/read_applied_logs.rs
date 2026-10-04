// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_log_filter, matches_log};
use crate::rpc::{
    encoding::{RpcBlockView, encode_rpc_block_logs, require_arity},
    errors::rpc_error,
    selectors::resolve_applied_selector,
};
use crate::sync::applied::{AppliedPublication, applied_commit};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;

pub(super) fn read_applied_logs(
    publication: &AppliedPublication,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 1, 1)?;
    let filter = decode_log_filter(&params[0])?;
    let commit = applied_commit(publication);
    if let Some(hash) = filter.block_hash {
        if hash != commit.target.execution_hash.0 {
            return Err(rpc_error(
                -32001,
                "GAP: log block is outside the captured applied view",
            ));
        }
    } else {
        resolve_applied_selector(
            publication,
            filter
                .from
                .as_ref()
                .unwrap_or(&Value::String("latest".into())),
        )?;
        resolve_applied_selector(
            publication,
            filter
                .to
                .as_ref()
                .unwrap_or(&Value::String("latest".into())),
        )?;
    }
    let logs = encode_rpc_block_logs(&RpcBlockView {
        version: &commit.target,
        block: &commit.block,
    })
    .map_err(|error| rpc_error(-32000, error.to_string()))?;
    let filtered = logs
        .into_iter()
        .filter(|log| matches_log(&filter, log))
        .collect::<Vec<_>>();
    serde_json::to_value(filtered).map_err(|error| rpc_error(-32603, error.to_string()))
}
