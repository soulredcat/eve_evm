// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::calculate_rewards::calculate_rewards;
use crate::rpc::{
    RpcContext,
    encoding::{parse_quantity, quantity, require_arity},
    errors::rpc_error,
    selectors::resolve_height,
};
use eve_protocol_config::headers::derive_next_base_fee;
use eve_storage::state::{capture_history_snapshot, read_history_block};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn read_fee_history(
    context: &RpcContext,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 2, 3)?;
    let count = u64::try_from(parse_quantity(&params[0])?)
        .map_err(|_| rpc_error(-32602, "history count overflow"))?;
    if count == 0 || count > 1000 {
        return Err(rpc_error(-32005, "fee history count must be1..1000"));
    }
    let percentiles = if let Some(value) = params.get(2) {
        super::decode_reward_percentiles::decode_reward_percentiles(value)?
    } else {
        Vec::new()
    };
    let history = capture_history_snapshot(&context.reader, context.history_budget)
        .map_err(|e| rpc_error(-32000, e.to_string()))?;
    let newest = resolve_height(&history, &params[1])?;
    if newest > history.version().height {
        return Err(rpc_error(-32001, "UNKNOWN_BLOCK"));
    }
    let oldest = newest.saturating_add(1).saturating_sub(count);
    let mut fees = Vec::new();
    let mut ratios = Vec::new();
    let mut rewards = Vec::new();
    for height in oldest..=newest {
        let block = read_history_block(&history, height)
            .map_err(|e| rpc_error(-32000, e.to_string()))?
            .ok_or_else(|| rpc_error(-32001, "history block unavailable"))?;
        fees.push(quantity(
            block
                .block
                .header
                .base_fee_per_gas
                .ok_or_else(|| rpc_error(-32000, "missing basefee"))?,
        ));
        ratios.push(block.block.header.gas_used as f64 / block.block.header.gas_limit as f64);
        if !percentiles.is_empty() {
            rewards.push(
                calculate_rewards(&block, &percentiles)
                    .map_err(|e| rpc_error(-32000, e.to_string()))?
                    .into_iter()
                    .map(quantity)
                    .collect::<Vec<_>>(),
            );
        }
        if height == newest {
            fees.push(quantity(
                derive_next_base_fee(&block.block.header)
                    .map_err(|e| rpc_error(-32000, format!("basefee: {e:?}")))?,
            ));
        }
    }
    let history = super::fee_types::FeeHistory {
        oldest_block: quantity(oldest),
        base_fee_per_gas: fees,
        gas_used_ratio: ratios,
        reward: params.get(2).map(|_| rewards),
    };
    serde_json::to_value(history).map_err(|e| rpc_error(-32603, e.to_string()))
}
