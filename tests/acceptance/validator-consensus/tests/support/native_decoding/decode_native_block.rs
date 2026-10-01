// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    NativeBlock, decode_base64::decode_base64, decode_native_block_id::decode_native_block_id,
    decode_native_commit::decode_native_commit, decode_native_header::decode_native_header,
};
use anyhow::{Result, ensure};
use eve_consensus_comet::consensus::certificates::{hash_consensus_header, hash_transaction_data};
use serde_json::Value;
/// Input is the entire /block result, preserving the actual current block ID outside result.block.
pub(crate) fn decode_native_block(result: &Value) -> Result<NativeBlock> {
    let block = &result["block"];
    ensure!(block.is_object(), "native block missing");
    let evidence = block["evidence"]
        .get("evidence")
        .ok_or_else(|| anyhow::anyhow!("native evidence array missing"))?;
    ensure!(
        evidence.is_null() || evidence.as_array().is_some_and(Vec::is_empty),
        "nonempty native evidence decoding unsupported"
    );
    let txs = block["data"]
        .get("txs")
        .ok_or_else(|| anyhow::anyhow!("native block transactions missing"))?;
    let mut transactions = Vec::new();
    if !txs.is_null() {
        let values = txs
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("native transaction array"))?;
        ensure!(
            values.len() <= 30_000_000 / 21_000,
            "native transaction count"
        );
        let mut total = 0_usize;
        for value in values {
            let transaction = decode_base64(value, 131_072, false)?;
            total = total
                .checked_add(transaction.len())
                .ok_or_else(|| anyhow::anyhow!("native transaction size overflow"))?;
            ensure!(total <= 4_194_304, "native block transaction bytes");
            transactions.push(transaction);
        }
    }
    let header = decode_native_header(&block["header"])?;
    let block_id = decode_native_block_id(&result["block_id"])?;
    ensure!(
        hash_consensus_header(&header)
            .map_err(|_| anyhow::anyhow!("native header invalid"))?
            .as_slice()
            == block_id.hash,
        "native header/current block ID mismatch"
    );
    ensure!(
        hash_transaction_data(&transactions)
            .map_err(|_| anyhow::anyhow!("native transaction data invalid"))?
            .as_slice()
            == header.data_hash,
        "native transaction/header data hash mismatch"
    );
    let last_commit = match block
        .get("last_commit")
        .ok_or_else(|| anyhow::anyhow!("native previous commit missing"))?
    {
        Value::Null => None,
        value => Some(decode_native_commit(value)?),
    };
    Ok(NativeBlock {
        header,
        last_commit,
        block_id,
        transactions,
    })
}
