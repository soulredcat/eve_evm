use super::StateService;
use crate::state::{ImmutableStateView, capture_state_snapshot, read_snapshot_commit};
use anyhow::{Context, Result, anyhow, ensure};
use std::sync::{Arc, atomic::Ordering};

/// Check complete cached state first; database loading occurs only after dropping cache guards.
pub fn read_state_service(service: &StateService) -> Result<Arc<ImmutableStateView>> {
    ensure!(
        !service.reader.fence.load(Ordering::Acquire),
        "state service fenced; reopen/reconcile required"
    );
    let publication = service
        .reader
        .publication
        .read()
        .map_err(|_| anyhow!("durable publication lock poisoned"))?
        .clone()
        .context("durable publication missing")?;
    {
        let cache = service
            .cache
            .read()
            .map_err(|_| anyhow!("state cache lock poisoned"))?;
        if let Some(view) = cache.as_ref()
            && view.database_sequence == publication.database_sequence
            && view.commit.target == publication.store_head
        {
            return Ok(Arc::clone(view));
        }
    }
    let snapshot = capture_state_snapshot(&service.reader)?;
    let commit = read_snapshot_commit(&snapshot, snapshot.version.height)?
        .context("complete state head missing")?;
    let view = Arc::new(ImmutableStateView {
        commit,
        database_sequence: snapshot.database_sequence,
    });
    let latest = service
        .reader
        .publication
        .read()
        .map_err(|_| anyhow!("durable publication lock poisoned"))?
        .clone()
        .context("durable publication missing")?;
    ensure!(
        latest.database_sequence == view.database_sequence
            && latest.store_head == view.commit.target,
        "state changed during cache refresh; retry query"
    );
    *service
        .cache
        .write()
        .map_err(|_| anyhow!("state cache lock poisoned"))? = Some(Arc::clone(&view));
    Ok(view)
}
