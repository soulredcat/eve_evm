use crate::state::{
    StateReader, StateSnapshot,
    encoding::{decode_head_marker, height_key},
    repository::validation::validate_snapshot_rows,
    types::{COMMIT_PREFIX, GENESIS_KEY, HEAD_KEY},
};
use anyhow::{Context, Result, anyhow, ensure};
use eve_state::{compute_commit_identity, decode_state_commit, encode_state_commit};
use std::sync::Arc;

pub(crate) fn initialize_state_snapshot(reader: &StateReader) -> Result<StateSnapshot<'_>> {
    let snapshot = reader.database.snapshot();
    let sequence = snapshot.sequence_number();
    let publication = reader
        .publication
        .read()
        .map_err(|_| anyhow!("durable publication lock poisoned"))?
        .clone()
        .context("durable state publication missing")?;
    ensure!(
        publication.database_sequence == sequence,
        "state write/publication in flight; retry snapshot capture"
    );
    let genesis = encode_state_commit(&reader.genesis, &reader.budget.logical)
        .map_err(|error| anyhow!("invalid stored genesis context: {error:?}"))?;
    ensure!(
        snapshot.get(GENESIS_KEY)?.as_deref() == Some(genesis.as_slice()),
        "snapshot genesis/config/schema/profile mismatch"
    );
    let (height, identity) = decode_head_marker(
        &snapshot
            .get(HEAD_KEY)?
            .context("snapshot durable head missing")?,
    )?;
    let bytes = snapshot
        .get(height_key(COMMIT_PREFIX, height))?
        .context("snapshot head payload missing")?;
    let commit = decode_state_commit(&bytes, &reader.budget.logical)
        .map_err(|error| anyhow!("invalid snapshot canonical head: {error:?}"))?;
    ensure!(
        commit.target.identity == reader.identity
            && commit.target == publication.store_head
            && identity == publication.commit_identity,
        "snapshot/publication/network version mismatch"
    );
    ensure!(
        compute_commit_identity(&commit, &reader.budget.logical)
            .map_err(|error| anyhow!("invalid snapshot identity: {error:?}"))?
            == identity,
        "snapshot durable marker/whole-commit mismatch"
    );
    validate_snapshot_rows(&snapshot, &commit, bytes, identity, true)?;
    Ok(StateSnapshot {
        snapshot,
        genesis: Arc::clone(&reader.genesis),
        budget: reader.budget,
        leases: Arc::clone(&reader.snapshots),
        version: commit.target,
        database_sequence: sequence,
    })
}
