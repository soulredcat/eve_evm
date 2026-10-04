// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::encoding::{RpcBlockView, encode_rpc_receipt};
use anyhow::{Context, Result};

pub(crate) fn calculate_rpc_rewards(
    block: &RpcBlockView<'_>,
    percentiles: &[f64],
) -> Result<Vec<u128>> {
    let mut samples = Vec::new();
    for index in 0..block.block.transactions.len() {
        let receipt = encode_rpc_receipt(block, index)?;
        let price = crate::rpc::encoding::parse_quantity(&receipt["effectiveGasPrice"])
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let gas = crate::rpc::encoding::parse_quantity(&receipt["gasUsed"])
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        samples.push((u128::try_from(price)?, u64::try_from(gas)?));
    }
    samples.sort_unstable_by_key(|(price, _)| *price);
    let base = u128::from(
        block
            .block
            .header
            .base_fee_per_gas
            .context("missing base fee")?,
    );
    Ok(percentiles
        .iter()
        .map(|percentile| {
            let target = (*percentile / 100.0 * block.block.header.gas_used as f64).ceil() as u64;
            let mut cumulative = 0;
            samples
                .iter()
                .find_map(|(price, gas)| {
                    cumulative += gas;
                    (cumulative >= target).then_some(price.saturating_sub(base))
                })
                .unwrap_or(0)
        })
        .collect())
}
