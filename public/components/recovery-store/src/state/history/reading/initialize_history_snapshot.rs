// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::{
    StateReader,
    encoding::decode_head_marker,
    history::{HistoryReadBudget, HistorySnapshot, keys::*},
    types::HEAD_KEY,
};
use anyhow::{Context, Result, anyhow, ensure};
use std::sync::Arc;
pub(crate) fn initialize_history_snapshot(
    reader: &StateReader,
    budget: HistoryReadBudget,
) -> Result<HistorySnapshot<'_>> {
    let snapshot = reader.database.snapshot();
    let sequence = snapshot.sequence_number();
    let publication = reader
        .publication
        .read()
        .map_err(|_| anyhow!("durable publication poisoned"))?
        .clone()
        .context("durable publication missing")?;
    ensure!(
        publication.database_sequence == sequence,
        "state publication in flight; retry"
    );
    ensure!(
        snapshot.get(INDEX_SCHEMA_KEY)?.as_deref() == Some(INDEX_SCHEMA),
        "HISTORY_NOT_READY: complete index unavailable"
    );
    let cursor = decode_head_marker(
        &snapshot
            .get(INDEX_CURSOR)?
            .context("HISTORY_NOT_READY: cursor missing")?,
    )?;
    let marker = decode_head_marker(&snapshot.get(HEAD_KEY)?.context("durable head missing")?)?;
    ensure!(
        cursor == marker && cursor == (publication.store_head.height, publication.commit_identity),
        "HISTORY_NOT_READY: stale index"
    );
    Ok(HistorySnapshot {
        snapshot,
        head: publication.store_head,
        database_sequence: sequence,
        storage_budget: reader.budget,
        read_budget: budget,
        leases: Arc::clone(&reader.snapshots),
    })
}
