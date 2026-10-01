// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::read_history_block;
use crate::state::history::{HistorySnapshot, keys::BLOCK_LOOKUP};
use alloy_primitives::B256;
use anyhow::{Context, Result, ensure};
pub fn lookup_execution_hash(snapshot: &HistorySnapshot<'_>, hash: B256) -> Result<Option<u64>> {
    let Some(bytes) = snapshot
        .snapshot
        .get([BLOCK_LOOKUP, hash.as_slice()].concat())?
    else {
        return Ok(None);
    };
    ensure!(bytes.len() == 8, "malformed block hash lookup");
    let height = u64::from_be_bytes(bytes.as_slice().try_into()?);
    let block = read_history_block(snapshot, height)?
        .context("block lookup points outside retained history")?;
    ensure!(
        block.version.execution_hash.0 == hash,
        "conflicting block hash lookup"
    );
    Ok(Some(height))
}
