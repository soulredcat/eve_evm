// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::advance_development_follower::advance_development_follower;
use crate::{
    mempool::MempoolHandle,
    sync::{
        applied::{
            AppliedOwner, AppliedReader, capture_applied_state, finish_applied_state_service,
        },
        follower::ValidatorFollowerSource,
    },
};
use anyhow::{Context, Result};
use std::time::Duration;
use tokio::sync::watch;

pub(super) async fn run_follower_actor(
    mut owner: AppliedOwner,
    reader: AppliedReader,
    pool: MempoolHandle,
    source: ValidatorFollowerSource,
    interval: Duration,
    mut shutdown: watch::Receiver<bool>,
) -> Result<()> {
    let mut error = None;
    loop {
        if *shutdown.borrow() {
            break;
        }
        let (next, advanced) =
            tokio::task::spawn_blocking(move || advance_development_follower(owner, source))
                .await
                .context("follower actor task failed")?;
        owner = next;
        match advanced {
            Ok(true) => {
                let publication = capture_applied_state(&reader)
                    .map_err(|failure| anyhow::anyhow!("follower publication: {failure:?}"))?;
                if let Err(failure) = pool.applied_committed(publication).await {
                    error = Some(anyhow::anyhow!("follower mempool stopped: {}", failure.0));
                    break;
                }
            }
            Ok(false) => {}
            Err(failure) => {
                error = Some(failure);
                break;
            }
        }
        tokio::select! { _ = tokio::time::sleep(interval) => {}, changed = shutdown.changed() => {
            if changed.is_err() || *shutdown.borrow() { break; }
        }}
    }
    let finished = tokio::task::spawn_blocking(move || finish_applied_state_service(owner))
        .await
        .context("follower shutdown task failed")?;
    if let Some(failure) = error {
        return Err(failure);
    }
    if let Some(failure) = finished.acknowledgement_error {
        anyhow::bail!("follower shutdown durability: {failure:?}");
    }
    finished
        .repository
        .map_err(|failure| anyhow::anyhow!("follower storage shutdown: {failure:?}"))?;
    Ok(())
}
