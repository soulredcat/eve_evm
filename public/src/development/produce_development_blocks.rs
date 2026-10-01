// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::commit_development_candidate::commit_development_candidate;
use crate::rpc::RpcContext;
use anyhow::{Result, anyhow};
use eve_storage::state::StateRepository;
use std::{
    sync::{Arc, atomic::Ordering},
    time::Duration,
};
use tokio::sync::watch;
pub(crate) async fn produce_development_blocks(
    mut repository: StateRepository,
    context: Arc<RpcContext>,
    interval_ms: u64,
    mut shutdown: watch::Receiver<bool>,
) -> Result<()> {
    let mut ticker = tokio::time::interval(Duration::from_millis(interval_ms));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        tokio::select! {_=shutdown.changed()=>return Ok(()),_=ticker.tick()=>{}}
        let candidates = context.pool.select().await.map_err(|e| anyhow!(e.0))?;
        if candidates.is_empty() {
            continue;
        }
        let shared = Arc::clone(&context);
        let (returned, head, event) = tokio::task::spawn_blocking(move || {
            commit_development_candidate(repository, shared, candidates)
        })
        .await
        .map_err(|_| anyhow!("producer worker failed"))??;
        repository = returned;
        if let Err(error) = context.pool.committed(head).await {
            context.healthy.store(false, Ordering::Release);
            return Err(anyhow!("synced block pool publication failed: {}", error.0));
        }
        let _ = context.events.send(event);
    }
}
