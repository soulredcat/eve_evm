// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::ReceiptEnvelope;
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::TxKind;
use alloy_rpc_types_eth::{Log, TransactionReceipt};
use anyhow::{Context, Result, ensure};
use eve_storage::state::RetainedBlockProjection;
use serde_json::Value;
pub(crate) fn encode_receipt(block: &RetainedBlockProjection, index: usize) -> Result<Value> {
    let raw = block
        .block
        .transactions
        .get(index)
        .context("receipt transaction missing")?;
    let validated =
        eve_evm::decode_signed_transaction(raw, block.version.identity.evm_chain_id, 131_072)
            .map_err(|e| anyhow::anyhow!("invalid stored transaction: {e:?}"))?;
    let mut prior_gas = 0;
    let mut log_index = 0_u64;
    for (current, bytes) in block.block.receipts.iter().enumerate().take(index + 1) {
        let mut remaining = bytes.as_ref();
        let receipt = ReceiptEnvelope::decode_2718(&mut remaining)?;
        ensure!(remaining.is_empty(), "trailing receipt bytes");
        let cumulative = receipt.cumulative_gas_used();
        ensure!(cumulative >= prior_gas, "decreasing receipt gas");
        if current != index {
            prior_gas = cumulative;
            log_index += u64::try_from(receipt.logs().len())?;
            continue;
        }
        let header = &block.block.header;
        let gas_price = validated
            .evm()
            .gas_priority_fee
            .map_or(validated.evm().gas_price, |tip| {
                validated
                    .evm()
                    .gas_price
                    .min(u128::from(header.base_fee_per_gas.unwrap_or(0)).saturating_add(tip))
            });
        let contract_address = match validated.evm().kind {
            TxKind::Create if receipt.status() => {
                Some(validated.sender().create(validated.evm().nonce))
            }
            _ => None,
        };
        let receipt = TransactionReceipt {
            inner: receipt,
            transaction_hash: validated.hash(),
            transaction_index: Some(u64::try_from(index)?),
            block_hash: Some(block.version.execution_hash.0),
            block_number: Some(block.version.height),
            gas_used: cumulative - prior_gas,
            effective_gas_price: gas_price,
            blob_gas_used: None,
            blob_gas_price: None,
            from: validated.sender(),
            to: validated.evm().kind.to().copied(),
            contract_address,
        }
        .map_logs(|inner| {
            let current = log_index;
            log_index += 1;
            Log {
                inner,
                block_hash: Some(block.version.execution_hash.0),
                block_number: Some(block.version.height),
                block_timestamp: Some(block.version.timestamp),
                transaction_hash: Some(validated.hash()),
                transaction_index: Some(index as u64),
                log_index: Some(current),
                removed: false,
            }
        });
        return Ok(serde_json::to_value(receipt)?);
    }
    anyhow::bail!("receipt index missing")
}
