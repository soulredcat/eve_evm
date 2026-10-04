// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::ReceiptEnvelope;
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::{Address, Bytes, keccak256};
use anyhow::{Context, Result, ensure};
use eve_state::StateCommit;
use serde_json::{Value, json};

pub(super) fn verify_public_receipt(
    receipt: &Value,
    commit: &StateCommit,
    transaction: &Bytes,
    sender: Address,
    recipient: Address,
) -> Result<()> {
    let index = commit
        .block
        .transactions
        .iter()
        .position(|raw| raw == transaction)
        .context("PUBLIC_FOLLOWER_ORACLE_TRANSACTION")?;
    let mut raw = commit.block.receipts[index].as_ref();
    let expected = ReceiptEnvelope::decode_2718(&mut raw)?;
    ensure!(
        raw.is_empty() && expected.status(),
        "PUBLIC_FOLLOWER_ORACLE_RECEIPT_STATUS"
    );
    let prior = if index == 0 {
        0
    } else {
        ReceiptEnvelope::decode_2718(&mut commit.block.receipts[index - 1].as_ref())?
            .cumulative_gas_used()
    };
    ensure!(
        receipt["transactionHash"] == json!(keccak256(transaction))
            && receipt["blockHash"] == json!(commit.target.execution_hash.0)
            && receipt["blockNumber"] == format!("0x{:x}", commit.target.height)
            && receipt["transactionIndex"] == format!("0x{index:x}")
            && receipt["from"] == json!(sender)
            && receipt["to"] == json!(recipient),
        "PUBLIC_FOLLOWER_RECEIPT_IDENTITY"
    );
    ensure!(
        receipt["status"] == "0x1"
            && receipt["cumulativeGasUsed"] == format!("0x{:x}", expected.cumulative_gas_used())
            && receipt["gasUsed"] == format!("0x{:x}", expected.cumulative_gas_used() - prior)
            && receipt["effectiveGasPrice"] == "0x77359400"
            && receipt["logs"]
                .as_array()
                .is_some_and(|logs| logs.is_empty()),
        "PUBLIC_FOLLOWER_RECEIPT_CONTENT"
    );
    Ok(())
}
