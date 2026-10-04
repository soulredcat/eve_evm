// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    calculate_rpc_rewards::calculate_rpc_rewards,
    decode_reward_percentiles::decode_reward_percentiles, fee_types::FeeHistory,
};
use crate::rpc::{
    encoding::{RpcBlockView, parse_quantity, quantity, require_arity},
    errors::rpc_error,
    selectors::resolve_applied_selector,
};
use crate::sync::applied::{AppliedPublication, applied_commit};
use eve_protocol_config::headers::derive_next_base_fee;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;

pub(super) fn read_applied_fee_history(
    publication: &AppliedPublication,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 2, 3)?;
    let count = u64::try_from(parse_quantity(&params[0])?)
        .map_err(|_| rpc_error(-32602, "history count overflow"))?;
    if count == 0 || count > 1000 {
        return Err(rpc_error(-32005, "fee history count must be1..1000"));
    }
    let newest = resolve_applied_selector(publication, &params[1])?;
    let oldest = newest.saturating_add(1).saturating_sub(count);
    if oldest != newest {
        return Err(rpc_error(
            -32001,
            "GAP: fee range is outside the captured applied view",
        ));
    }
    let percentiles = params
        .get(2)
        .map(decode_reward_percentiles)
        .transpose()?
        .unwrap_or_default();
    let commit = applied_commit(publication);
    let view = RpcBlockView {
        version: &commit.target,
        block: &commit.block,
    };
    let header = &commit.block.header;
    let rewards = if percentiles.is_empty() {
        Vec::new()
    } else {
        vec![
            calculate_rpc_rewards(&view, &percentiles)
                .map_err(|error| rpc_error(-32000, error.to_string()))?
                .into_iter()
                .map(quantity)
                .collect(),
        ]
    };
    let response = FeeHistory {
        oldest_block: quantity(oldest),
        base_fee_per_gas: vec![
            quantity(
                header
                    .base_fee_per_gas
                    .ok_or_else(|| rpc_error(-32000, "missing basefee"))?,
            ),
            quantity(
                derive_next_base_fee(header)
                    .map_err(|error| rpc_error(-32000, format!("basefee: {error:?}")))?,
            ),
        ],
        gas_used_ratio: vec![header.gas_used as f64 / header.gas_limit as f64],
        reward: params.get(2).map(|_| rewards),
    };
    serde_json::to_value(response).map_err(|error| rpc_error(-32603, error.to_string()))
}
