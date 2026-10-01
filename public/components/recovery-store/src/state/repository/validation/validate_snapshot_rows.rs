// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::load_block_hash_history::load_block_hash_history;
use crate::state::{
    encoding::{build_current_state_entries, build_historical_entries},
    types::{ACCOUNT_PREFIX, CODE_PREFIX, HASH_PREFIX, SLOT_PREFIX, SYSTEM_PREFIX},
};
use alloy_primitives::B256;
use anyhow::{Context, Result, ensure};
use eve_state::StateCommit;
use eve_state::validate_block_hash_history;
use rocksdb::{DB, Direction, IteratorMode, SnapshotWithThreadMode};

pub(crate) fn validate_snapshot_rows(
    snapshot: &SnapshotWithThreadMode<'_, DB>,
    commit: &StateCommit,
    bytes: Vec<u8>,
    identity: B256,
    current: bool,
) -> Result<()> {
    validate_block_hash_history(commit, &load_block_hash_history(snapshot, commit)?).map_err(
        |error| anyhow::anyhow!("invalid complete historical block-hash view: {error:?}"),
    )?;
    for (key, value) in build_historical_entries(commit, bytes, identity) {
        ensure!(
            snapshot.get(key)?.as_deref() == Some(value.as_slice()),
            "missing/mismatched historical block/root/recovery reference"
        );
    }
    for (hash, code) in &commit.state.codes {
        ensure!(
            snapshot
                .get([CODE_PREFIX, hash.as_slice()].concat())?
                .as_deref()
                == Some(code.as_ref()),
            "missing/mismatched content-addressed code blob"
        );
    }
    if current {
        let expected = build_current_state_entries(commit)?;
        let mut observed = 0usize;
        for prefix in [ACCOUNT_PREFIX, SLOT_PREFIX, SYSTEM_PREFIX, HASH_PREFIX] {
            for row in snapshot.iterator(IteratorMode::From(prefix, Direction::Forward)) {
                let (key, value) = row?;
                if !key.starts_with(prefix) {
                    break;
                }
                observed = observed
                    .checked_add(1)
                    .context("state row count overflow")?;
                ensure!(
                    expected.get(key.as_ref()).map(Vec::as_slice) == Some(value.as_ref()),
                    "unexpected or corrupted current state row"
                );
            }
        }
        ensure!(
            observed == expected.len(),
            "missing current account/slot/system/hash rows"
        );
    }
    Ok(())
}
