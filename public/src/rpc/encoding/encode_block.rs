// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{encode_transaction, measure_block_size::measure_block_size, quantity};
use alloy_primitives::keccak256;
use anyhow::{Result, ensure};
use eve_storage::state::RetainedBlockProjection;
use serde_json::Value;
pub(crate) fn encode_block(block: &RetainedBlockProjection, full: bool) -> Result<Value> {
    let mut value =
        serde_json::to_value(alloy_rpc_types_eth::Header::new(block.block.header.clone()))?;
    let mut transactions = Vec::new();
    let mut bytes = 4096_usize;
    for (index, raw) in block.block.transactions.iter().enumerate() {
        let transaction = if full {
            encode_transaction(
                raw,
                block.version.identity.evm_chain_id,
                Some(&block.block.header),
                Some(u64::try_from(index)?),
            )?
        } else {
            serde_json::to_value(keccak256(raw))?
        };
        bytes = bytes
            .checked_add(serde_json::to_vec(&transaction)?.len() + 1)
            .ok_or_else(|| anyhow::anyhow!("block response accounting overflow"))?;
        ensure!(
            bytes <= 4 * 1_048_576,
            "block response byte capacity exceeded"
        );
        transactions.push(transaction);
    }
    value["transactions"] = Value::Array(transactions);
    value["uncles"] = Value::Array(Vec::new());
    value["withdrawals"] = Value::Array(Vec::new());
    value["size"] = quantity(measure_block_size(block)?);
    Ok(value)
}
