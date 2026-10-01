// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::initialize_history_snapshot::initialize_history_snapshot;
use crate::state::{
    StateReader,
    history::{HistoryReadBudget, HistorySnapshot},
};
use anyhow::{Result, anyhow, ensure};
use std::sync::atomic::Ordering;
pub fn capture_history_snapshot(
    reader: &StateReader,
    budget: HistoryReadBudget,
) -> Result<HistorySnapshot<'_>> {
    ensure!(
        budget.maximum_block_bytes > 0
            && budget.maximum_block_bytes <= reader.budget.maximum_commit_bytes,
        "invalid history read budget"
    );
    ensure!(!reader.fence.load(Ordering::Acquire), "state reader fenced");
    reader
        .snapshots
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
            (count < reader.budget.maximum_snapshots).then_some(count + 1)
        })
        .map_err(|_| anyhow!("history snapshot capacity exceeded"))?;
    let result = initialize_history_snapshot(reader, budget);
    if result.is_err() {
        reader.snapshots.fetch_sub(1, Ordering::AcqRel);
    }
    result
}
