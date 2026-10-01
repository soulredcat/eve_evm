// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::validate_index_row::validate_index_row;
use crate::state::{
    encoding::{decode_head_marker, height_key},
    history::keys::*,
    types::ID_PREFIX,
};
use alloy_primitives::B256;
use anyhow::{Context, Result, ensure};
use eve_state::StateVersion;
use rocksdb::{DB, Direction, IteratorMode, SnapshotWithThreadMode};
pub(crate) fn validate_index_cursor(
    snapshot: &SnapshotWithThreadMode<'_, DB>,
    head: &StateVersion,
    identity: B256,
) -> Result<()> {
    let schema = snapshot.get(INDEX_SCHEMA_KEY)?;
    let cursor = snapshot.get(INDEX_CURSOR)?;
    if schema.is_none() && cursor.is_none() {
        for prefix in [BLOCK_LOOKUP, TX_LOOKUP, VERSION_PREFIX] {
            if let Some(row) = snapshot
                .iterator(IteratorMode::From(prefix, Direction::Forward))
                .next()
            {
                let (key, _) = row?;
                ensure!(
                    !key.starts_with(prefix),
                    "orphan derived index entries without cursor"
                );
            }
        }
        return Ok(());
    }
    if let Some(schema) = schema.as_deref() {
        ensure!(schema == INDEX_SCHEMA, "unsupported history schema");
    }
    let (height, source) = decode_head_marker(&cursor.context("history cursor missing")?)?;
    ensure!(
        height <= head.height
            && snapshot.get(height_key(ID_PREFIX, height))?.as_deref() == Some(source.as_slice()),
        "history cursor source mismatch"
    );
    if schema.is_some() {
        ensure!(
            height == head.height && source == identity,
            "complete history index is stale"
        );
    }
    for prefix in [BLOCK_LOOKUP, TX_LOOKUP, VERSION_PREFIX] {
        for row in snapshot.iterator(IteratorMode::From(prefix, Direction::Forward)) {
            let (key, value) = row?;
            if !key.starts_with(prefix) {
                break;
            }
            validate_index_row(snapshot, &key, &value, height)?;
        }
    }
    Ok(())
}
