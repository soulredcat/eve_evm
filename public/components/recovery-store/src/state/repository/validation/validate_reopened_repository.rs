use crate::state::{
    CommitDisposition, DurableStateAck, StateRepository,
    encoding::{decode_head_marker, height_key},
    repository::validation::validate_snapshot_rows,
    types::{COMMIT_PREFIX, GENESIS_KEY, HEAD_KEY},
};
use anyhow::{Context, Result, anyhow, ensure};
use eve_state::{compute_commit_identity, decode_state_commit, encode_state_commit};

pub(crate) fn validate_reopened_repository(store: &StateRepository) -> Result<DurableStateAck> {
    let snapshot = store.database.snapshot();
    let expected_genesis = encode_state_commit(&store.genesis, &store.budget.logical)
        .map_err(|error| anyhow!("invalid supplied genesis: {error:?}"))?;
    ensure!(
        snapshot.get(GENESIS_KEY)?.as_deref() == Some(expected_genesis.as_slice()),
        "full-state genesis/config/schema/profile mismatch"
    );
    let (height, head_identity) = decode_head_marker(
        &snapshot
            .get(HEAD_KEY)?
            .context("full-state durable marker missing")?,
    )?;
    let mut parent = None;
    let mut head = None;
    for current in 0..=height {
        let bytes = snapshot
            .get(height_key(COMMIT_PREFIX, current))?
            .context("missing retained recovery commit")?;
        let commit = decode_state_commit(&bytes, &store.budget.logical).map_err(|error| {
            anyhow!("corrupt canonical recovery commit at {current}: {error:?}")
        })?;
        ensure!(
            commit.target.height == current
                && commit.target.identity == store.identity
                && commit.parent == parent,
            "invalid retained parent/network/config/profile chain"
        );
        let identity = compute_commit_identity(&commit, &store.budget.logical)
            .map_err(|error| anyhow!("invalid recovery identity: {error:?}"))?;
        validate_snapshot_rows(&snapshot, &commit, bytes, identity, current == height)?;
        if current == height {
            ensure!(
                identity == head_identity,
                "durable marker/whole-commit identity mismatch"
            );
        }
        parent = Some(commit.target.clone());
        head = Some(commit);
    }
    let head = head.context("empty full-state recovery chain")?;
    Ok(DurableStateAck {
        committed: head.target.clone(),
        store_head: head.target,
        commit_identity: head_identity,
        database_sequence: snapshot.sequence_number(),
        disposition: CommitDisposition::ExactReplay,
    })
}
