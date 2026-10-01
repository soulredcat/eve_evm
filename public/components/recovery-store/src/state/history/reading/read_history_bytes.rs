// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::{encoding::height_key, history::HistorySnapshot};
use anyhow::{Context, Result, ensure};
/// Count all retained row value bytes in the selected projection before decoding.
pub(crate) fn read_history_bytes(
    snapshot: &HistorySnapshot<'_>,
    prefix: &[u8],
    height: u64,
    used: &mut usize,
) -> Result<Vec<u8>> {
    let bytes = snapshot
        .snapshot
        .get(height_key(prefix, height))?
        .context("missing retained history row")?;
    *used = used
        .checked_add(bytes.len())
        .context("history byte accounting overflow")?;
    ensure!(
        *used <= snapshot.read_budget.maximum_block_bytes,
        "history block byte capacity exceeded"
    );
    Ok(bytes)
}
