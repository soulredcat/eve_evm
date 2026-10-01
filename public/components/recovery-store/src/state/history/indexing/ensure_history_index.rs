// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::encode_index_entries::encode_index_entries;
use crate::state::{
    StateRepository,
    encoding::{decode_head_marker, height_key},
    history::{
        HistoryIndexStatus, HistoryReadBudget,
        keys::*,
        validation::{validate_index_cursor, validate_indexed_commit},
    },
    types::{COMMIT_PREFIX, HEAD_KEY, ID_PREFIX},
};
use anyhow::{Context, Result, anyhow, ensure};
use eve_state::{compute_commit_identity, decode_state_commit};
use rocksdb::{WriteBatch, WriteOptions};
use std::{collections::BTreeMap, sync::atomic::Ordering};
/// Derive auxiliary lookups in bounded durable passes; canonical B1 bytes stay unchanged.
pub fn ensure_history_index(
    store: &mut StateRepository,
    limits: HistoryReadBudget,
) -> Result<HistoryIndexStatus> {
    ensure!(
        limits.maximum_rebuild_blocks > 0 && limits.maximum_index_batch_bytes > 0,
        "empty history bootstrap budget"
    );
    ensure!(
        limits.maximum_index_batch_bytes <= store.budget.maximum_commit_bytes
            && limits.maximum_index_batch_bytes <= store.budget.database.max_batch_bytes,
        "history pass exceeds repository physical batch bound"
    );
    ensure!(!store.fence.load(Ordering::Acquire), "state store fenced");
    let snapshot = store.database.snapshot();
    let (target, target_id) =
        decode_head_marker(&snapshot.get(HEAD_KEY)?.context("durable head missing")?)?;
    let publication = store
        .publication
        .read()
        .map_err(|_| anyhow!("durable publication poisoned"))?
        .clone()
        .context("durable head missing")?;
    ensure!(
        snapshot.sequence_number() == publication.database_sequence,
        "index source not acknowledged"
    );
    validate_index_cursor(&snapshot, &publication.store_head, target_id)?;
    let previous = snapshot
        .get(INDEX_CURSOR)?
        .map(|bytes| decode_head_marker(&bytes))
        .transpose()?;
    if previous == Some((target, target_id))
        && snapshot.get(INDEX_SCHEMA_KEY)?.as_deref() == Some(INDEX_SCHEMA)
    {
        return Ok(HistoryIndexStatus {
            indexed_height: Some(target),
            target_height: target,
            complete: true,
        });
    }
    let start = previous.map_or(Some(0), |(height, _)| height.checked_add(1));
    let mut batch = WriteBatch::default();
    let mut queued = BTreeMap::new();
    let mut indexed = previous.map(|(height, _)| height);
    for (processed, height) in start
        .into_iter()
        .flat_map(|start| start..=target)
        .enumerate()
    {
        if processed >= limits.maximum_rebuild_blocks {
            break;
        }
        let bytes = snapshot
            .get(height_key(COMMIT_PREFIX, height))?
            .context("missing retained bootstrap source")?;
        let commit = decode_state_commit(&bytes, &store.budget.logical)
            .map_err(|e| anyhow!("invalid retained source: {e:?}"))?;
        let identity = compute_commit_identity(&commit, &store.budget.logical)
            .map_err(|e| anyhow!("invalid source identity: {e:?}"))?;
        ensure!(
            commit.target.height == height
                && commit.target.identity == store.identity
                && snapshot.get(height_key(ID_PREFIX, height))?.as_deref()
                    == Some(identity.as_slice()),
            "bootstrap source mismatch"
        );
        validate_indexed_commit(&snapshot, &commit, identity)?;
        let entries = encode_index_entries(&commit, identity)?;
        let mut candidate = WriteBatch::default();
        for (key, value) in &entries {
            candidate.put(key, value);
        }
        if height == target {
            candidate.put(INDEX_SCHEMA_KEY, INDEX_SCHEMA);
        }
        let overhead = WriteBatch::default().size_in_bytes();
        let contribution = candidate
            .size_in_bytes()
            .checked_sub(overhead)
            .context("index batch header mismatch")?;
        let prospective = batch
            .size_in_bytes()
            .checked_add(contribution)
            .context("index pass accounting overflow")?;
        if prospective > limits.maximum_index_batch_bytes {
            ensure!(
                processed > 0,
                "single history index block exceeds pass byte budget"
            );
            break;
        }
        for (key, value) in entries {
            if key != INDEX_CURSOR {
                if let Some(existing) = snapshot.get(&key)? {
                    ensure!(existing == value, "conflicting existing history entry");
                }
                if let Some(existing) = queued.insert(key.clone(), value.clone()) {
                    ensure!(existing == value, "conflicting bootstrap identity");
                }
            }
            batch.put(key, value);
        }
        ensure!(
            batch.size_in_bytes() <= limits.maximum_index_batch_bytes,
            "history pass byte budget exceeded"
        );
        indexed = Some(height);
    }
    if indexed == Some(target) {
        batch.put(INDEX_SCHEMA_KEY, INDEX_SCHEMA);
    }
    ensure!(
        batch.size_in_bytes() <= limits.maximum_index_batch_bytes,
        "final schema-inclusive history batch exceeds byte budget"
    );
    drop(snapshot);
    let mut options = WriteOptions::default();
    options.set_sync(true);
    options.disable_wal(false);
    #[cfg(test)]
    if let Some(crate::state::types::SimulatedCommitFailure::BeforeWrite) =
        store.simulated_index_failure
    {
        store.simulated_index_failure = None;
        store.fence.store(true, Ordering::Release);
        return Err(anyhow!(
            "SIMULATED ambiguous auxiliary pre-write failure; no hardware fault claimed"
        ));
    }
    if let Err(error) = store.database.write_opt(batch, &options) {
        store.fence.store(true, Ordering::Release);
        return Err(anyhow!("ambiguous index sync; store fenced: {error}"));
    }
    #[cfg(test)]
    if let Some(crate::state::types::SimulatedCommitFailure::AfterSuccessfulSync) =
        store.simulated_index_failure
    {
        store.simulated_index_failure = None;
        store.fence.store(true, Ordering::Release);
        return Err(anyhow!(
            "SIMULATED auxiliary lost acknowledgment after actual successful sync"
        ));
    }
    match store.publication.write() {
        Ok(mut publication) => {
            let Some(publication) = publication.as_mut() else {
                store.fence.store(true, Ordering::Release);
                return Err(anyhow!("index publication missing; store fenced"));
            };
            publication.database_sequence = store.database.latest_sequence_number();
        }
        Err(_) => {
            store.fence.store(true, Ordering::Release);
            return Err(anyhow!("index publication failed; store fenced"));
        }
    }
    Ok(HistoryIndexStatus {
        indexed_height: indexed,
        target_height: target,
        complete: indexed == Some(target),
    })
}
