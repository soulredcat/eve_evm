// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::{
    StateRepository,
    encoding::{build_current_state_entries, build_historical_entries, encode_head_marker},
    types::{ACCOUNT_PREFIX, CODE_PREFIX, HASH_PREFIX, HEAD_KEY, SLOT_PREFIX, SYSTEM_PREFIX},
};
use alloy_primitives::B256;
use anyhow::{Result, ensure};
use eve_state::StateCommit;
use rocksdb::{Direction, IteratorMode, WriteBatch};

pub(crate) fn build_state_batch(
    store: &StateRepository,
    commit: &StateCommit,
    bytes: Vec<u8>,
    identity: B256,
) -> Result<WriteBatch> {
    let snapshot = store.database.snapshot();
    let mut batch = WriteBatch::default();
    for prefix in [ACCOUNT_PREFIX, SLOT_PREFIX, SYSTEM_PREFIX, HASH_PREFIX] {
        for row in snapshot.iterator(IteratorMode::From(prefix, Direction::Forward)) {
            let (key, _) = row?;
            if !key.starts_with(prefix) {
                break;
            }
            batch.delete(key);
            ensure!(
                batch.size_in_bytes() <= store.budget.maximum_commit_bytes,
                "state deletion batch exceeds byte budget"
            );
        }
    }
    for (key, value) in build_current_state_entries(commit)? {
        batch.put(key, value);
    }
    for (hash, code) in &commit.state.codes {
        let key = [CODE_PREFIX, hash.as_slice()].concat();
        if let Some(existing) = snapshot.get(&key)? {
            ensure!(
                existing == code.as_ref(),
                "content-addressed code identity conflict"
            );
        }
        batch.put(key, code.as_ref());
    }
    for (key, value) in build_historical_entries(commit, bytes, identity) {
        batch.put(key, value);
    }
    batch.put(HEAD_KEY, encode_head_marker(commit.target.height, identity));
    crate::state::history::indexing::append_history_index(store, &mut batch, commit, identity)?;
    ensure!(
        batch.size_in_bytes() <= store.budget.maximum_commit_bytes,
        "atomic state/block/marker batch exceeds byte budget"
    );
    Ok(batch)
}
