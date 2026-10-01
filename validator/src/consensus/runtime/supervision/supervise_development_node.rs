// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{consensus::runtime::types::RunningNode, development::engine::engine_child};
use anyhow::{Result, ensure};
use std::{
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

pub(in crate::consensus::runtime) async fn supervise_development_node(
    node: &RunningNode,
) -> Result<()> {
    let mut termination =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut interrupt = Box::pin(tokio::signal::ctrl_c());
    let mut tick = tokio::time::interval(Duration::from_millis(50));
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut ready = false;
    loop {
        tokio::select! {
            signal = &mut interrupt => { signal?; return Ok(()); },
            _ = termination.recv() => return Ok(()),
            _ = tick.tick() => {}
        }
        if node.stop.load(Ordering::Acquire) {
            return Err(node
                .failure
                .lock()
                .map_err(|_| anyhow::anyhow!("channel failure lock poisoned"))?
                .take()
                .unwrap_or_else(|| {
                    anyhow::anyhow!("node worker stopped without verified shutdown")
                }));
        }
        {
            let mut engine = node
                .engine
                .lock()
                .map_err(|_| anyhow::anyhow!("engine supervisor lock poisoned"))?;
            engine_child(&mut engine)?;
        }
        if !ready {
            ready = super::emit_node_readiness::emit_node_readiness(node)?;
            ensure!(
                ready || Instant::now() < deadline,
                "actual native application/signer readiness deadline exceeded"
            );
        }
    }
}
