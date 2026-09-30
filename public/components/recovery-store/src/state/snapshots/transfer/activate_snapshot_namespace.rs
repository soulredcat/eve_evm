use super::{
    sync_snapshot_directory::sync_snapshot_directory,
    validate_snapshot_manifest::validate_snapshot_manifest,
};
use crate::state::{
    StateRepository, StateStorageBudget, commit_state, open_state_repository,
    repository::open_state_namespace, types::ACTIVE_KEY,
};
use anyhow::{Context, Result, ensure};
use eve_state::StateCommit;
use rocksdb::{WriteBatch, WriteOptions};
use std::path::Path;

/// Activate locally checked recovery data into a NEW namespace, never overwrite the source/live store.
pub fn activate_snapshot_namespace(
    source: &Path,
    destination: &Path,
    genesis: &StateCommit,
    budget: StateStorageBudget,
) -> Result<StateRepository> {
    let (manifest, commits) = validate_snapshot_manifest(source, genesis, &budget)?;
    std::fs::create_dir(destination).context("snapshot activation requires a NEW namespace")?;
    let mut staging = open_state_namespace(destination, genesis, budget, false)?;
    for commit in commits.iter().skip(1) {
        commit_state(&mut staging, commit)?;
    }
    let head = staging
        .publication
        .read()
        .map_err(|_| anyhow::anyhow!("staging publication lock poisoned"))?
        .clone()
        .context("staging durable head missing")?;
    ensure!(
        head.store_head.height == manifest.height
            && head.commit_identity.0 == manifest.head_commit_identity,
        "staged snapshot head does not match complete manifest"
    );
    let mut batch = WriteBatch::default();
    batch.put(ACTIVE_KEY, [1]);
    let mut writes = WriteOptions::default();
    writes.set_sync(true);
    writes.disable_wal(false);
    staging
        .database
        .write_opt(batch, &writes)
        .context("sync NEW snapshot namespace activation")?;
    drop(staging);
    sync_snapshot_directory(destination)?;
    open_state_repository(destination, genesis, budget)
}
