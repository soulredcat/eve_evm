// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::read_history_block;
use crate::state::history::{HistorySnapshot, TransactionLocation, keys::TX_LOOKUP};
use alloy_primitives::{B256, keccak256};
use anyhow::{Context, Result, ensure};
pub fn lookup_transaction(
    snapshot: &HistorySnapshot<'_>,
    hash: B256,
) -> Result<Option<TransactionLocation>> {
    let Some(bytes) = snapshot
        .snapshot
        .get([TX_LOOKUP, hash.as_slice()].concat())?
    else {
        return Ok(None);
    };
    ensure!(bytes.len() == 12, "malformed transaction lookup");
    let height = u64::from_be_bytes(bytes[..8].try_into()?);
    let transaction_index = u32::from_be_bytes(bytes[8..].try_into()?);
    let block = read_history_block(snapshot, height)?
        .context("transaction lookup outside retained history")?;
    let tx = block
        .block
        .transactions
        .get(transaction_index as usize)
        .context("transaction lookup index invalid")?;
    ensure!(keccak256(tx) == hash, "conflicting transaction lookup");
    Ok(Some(TransactionLocation {
        height,
        transaction_index,
    }))
}
