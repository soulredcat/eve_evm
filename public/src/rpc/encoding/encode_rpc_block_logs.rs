// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::estimate_json_value_bytes;
use super::rpc_block_view::RpcBlockView;
use alloy_consensus::ReceiptEnvelope;
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::keccak256;
use alloy_rpc_types_eth::Log;
use anyhow::{Result, ensure};
use serde_json::Value;
pub(crate) fn encode_rpc_block_logs(block: &RpcBlockView<'_>) -> Result<Vec<Value>> {
    let mut logs = Vec::new();
    let mut bytes = 2_usize;
    let mut log_index = 0_u64;
    for (transaction_index, (raw, receipt)) in block
        .block
        .transactions
        .iter()
        .zip(&block.block.receipts)
        .enumerate()
    {
        let mut remaining = receipt.as_ref();
        let receipt = ReceiptEnvelope::decode_2718(&mut remaining)?;
        ensure!(remaining.is_empty(), "trailing receipt bytes");
        for inner in receipt.logs() {
            let log = Log {
                inner: inner.clone(),
                block_hash: Some(block.version.execution_hash.0),
                block_number: Some(block.version.height),
                block_timestamp: Some(block.version.timestamp),
                transaction_hash: Some(keccak256(raw)),
                transaction_index: Some(u64::try_from(transaction_index)?),
                log_index: Some(log_index),
                removed: false,
            };
            let value = serde_json::to_value(log)?;
            bytes = bytes
                .checked_add(
                    estimate_json_value_bytes(&value, 4 * 1_048_576)
                        .map_err(|e| anyhow::anyhow!(e.to_string()))?
                        + 1,
                )
                .ok_or_else(|| anyhow::anyhow!("log byte accounting overflow"))?;
            ensure!(
                bytes <= 4 * 1_048_576 && logs.len() < 10_000,
                "log result byte/count capacity exceeded; narrow HTTP range"
            );
            logs.push(value);
            log_index += 1;
        }
    }
    Ok(logs)
}
