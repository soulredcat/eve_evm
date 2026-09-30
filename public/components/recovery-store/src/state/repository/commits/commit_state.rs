use super::build_state_batch::build_state_batch;
use crate::state::{
    CommitDisposition, DurableStateAck, StateRepository, encoding::height_key, types::COMMIT_PREFIX,
};
use anyhow::{Context, Result, anyhow, ensure};
use eve_state::{
    StateCommit, compute_commit_identity, encode_state_commit, validate_block_hash_history,
};
use rocksdb::WriteOptions;
use std::sync::atomic::Ordering;

/// Atomically sync every state/recovery reference. An ambiguous I/O error fences all fresh reads/writes.
pub fn commit_state(store: &mut StateRepository, commit: &StateCommit) -> Result<DurableStateAck> {
    ensure!(
        !store.fence.load(Ordering::Acquire),
        "state handle fenced; drop all readers and reopen/reconcile"
    );
    ensure!(
        commit.target.identity == store.identity,
        "wrong genesis/network/config/profile commit"
    );
    let bytes = encode_state_commit(commit, &store.budget.logical)
        .map_err(|error| anyhow!("invalid canonical commit: {error:?}"))?;
    let identity = compute_commit_identity(commit, &store.budget.logical)
        .map_err(|error| anyhow!("invalid canonical identity: {error:?}"))?;
    let head = store
        .publication
        .read()
        .map_err(|_| anyhow!("durable publication lock poisoned"))?
        .clone()
        .context("durable publication missing")?;
    if commit.target.height <= head.store_head.height {
        ensure!(
            store
                .database
                .get(height_key(COMMIT_PREFIX, commit.target.height))?
                .as_deref()
                == Some(bytes.as_slice()),
            "replay differs from exact whole retained commit"
        );
        return Ok(DurableStateAck {
            committed: commit.target.clone(),
            store_head: head.store_head,
            commit_identity: identity,
            database_sequence: head.database_sequence,
            disposition: CommitDisposition::ExactReplay,
        });
    }
    ensure!(
        commit.parent.as_ref() == Some(&head.store_head),
        "stale/mismatched full-state parent"
    );
    let history = crate::state::repository::validation::load_block_hash_history(
        &store.database.snapshot(),
        commit,
    )?;
    validate_block_hash_history(commit, &history)
        .map_err(|error| anyhow!("wrong/missing historical execution hash: {error:?}"))?;
    let batch = build_state_batch(store, commit, bytes, identity)?;
    let mut writes = WriteOptions::default();
    writes.set_sync(true);
    writes.disable_wal(false);
    #[cfg(test)]
    if let Some(crate::state::types::SimulatedCommitFailure::BeforeWrite) = store.simulated_failure
    {
        store.simulated_failure = None;
        store.fence.store(true, Ordering::Release);
        return Err(anyhow!(
            "SIMULATED pre-write ambiguous error; no hardware failure claimed"
        ));
    }
    if let Err(error) = store.database.write_opt(batch, &writes) {
        store.fence.store(true, Ordering::Release);
        return Err(anyhow!(
            "ambiguous state write/sync outcome; handle fenced: {error}"
        ));
    }
    #[cfg(test)]
    if let Some(crate::state::types::SimulatedCommitFailure::AfterSuccessfulSync) =
        store.simulated_failure
    {
        store.simulated_failure = None;
        store.fence.store(true, Ordering::Release);
        return Err(anyhow!(
            "SIMULATED lost acknowledgment after actual successful sync"
        ));
    }
    let ack = DurableStateAck {
        committed: commit.target.clone(),
        store_head: commit.target.clone(),
        commit_identity: identity,
        database_sequence: store.database.latest_sequence_number(),
        disposition: CommitDisposition::NewlySynced,
    };
    match store.publication.write() {
        Ok(mut publication) => *publication = Some(ack.clone()),
        Err(_) => {
            store.fence.store(true, Ordering::Release);
            return Err(anyhow!("synced state publication failed; handle fenced"));
        }
    }
    Ok(ack)
}
