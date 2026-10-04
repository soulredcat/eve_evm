// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{
    encoding::{RpcBlockView, encode_rpc_block, parse_fixed, require_arity},
    errors::rpc_error,
    selectors::resolve_applied_selector,
};
use crate::sync::applied::{AppliedPublication, applied_commit};
use alloy_primitives::B256;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;

pub(super) fn read_applied_block(
    publication: &AppliedPublication,
    params: &[Value],
    by_hash: bool,
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 2, 2)?;
    let full = params[1]
        .as_bool()
        .ok_or_else(|| rpc_error(-32602, "full transactions must be boolean"))?;
    let commit = applied_commit(publication);
    if by_hash {
        if B256::from(parse_fixed::<32>(&params[0])?) != commit.target.execution_hash.0 {
            return Err(rpc_error(
                -32001,
                "GAP: block hash is outside the captured applied view",
            ));
        }
    } else {
        resolve_applied_selector(publication, &params[0])?;
    }
    encode_rpc_block(
        &RpcBlockView {
            version: &commit.target,
            block: &commit.block,
        },
        full,
    )
    .map_err(|error| rpc_error(-32000, error.to_string()))
}
