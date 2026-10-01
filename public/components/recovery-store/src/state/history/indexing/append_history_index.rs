// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::encode_index_entries::encode_index_entries;
use crate::state::{
    StateRepository,
    encoding::decode_head_marker,
    history::keys::{INDEX_CURSOR, INDEX_SCHEMA, INDEX_SCHEMA_KEY},
};
use alloy_primitives::B256;
use anyhow::{Context, Result, ensure};
use eve_state::StateCommit;
use rocksdb::WriteBatch;

pub(crate) fn append_history_index(
    store: &StateRepository,
    batch: &mut WriteBatch,
    commit: &StateCommit,
    identity: B256,
) -> Result<()> {
    let Some(schema) = store.database.get(INDEX_SCHEMA_KEY)? else {
        return Ok(());
    };
    ensure!(
        schema == INDEX_SCHEMA,
        "unsupported derived history index schema"
    );
    let (height, indexed) = decode_head_marker(
        &store
            .database
            .get(INDEX_CURSOR)?
            .context("history index cursor missing")?,
    )?;
    let parent = commit
        .parent
        .as_ref()
        .context("history-indexed successor requires parent")?;
    let published = store
        .publication
        .read()
        .map_err(|_| anyhow::anyhow!("durable publication poisoned"))?
        .clone()
        .context("durable head missing")?;
    ensure!(
        height == parent.height && indexed == published.commit_identity,
        "history index is incomplete or mismatched"
    );
    for (key, value) in encode_index_entries(commit, identity)? {
        if key != INDEX_CURSOR
            && let Some(existing) = store.database.get(&key)?
        {
            ensure!(
                existing == value,
                "conflicting derived history reverse lookup"
            );
        }
        batch.put(key, value);
    }
    Ok(())
}
