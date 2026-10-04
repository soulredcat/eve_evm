// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DevelopmentFollowerConfig, open_development_follower::open_development_follower,
    run_follower_actor::run_follower_actor,
};
use crate::{
    mempool::{MempoolLimits, start_applied_mempool},
    rpc::{AppliedRpcConfig, RpcListenerAddresses, create_applied_rpc_context},
    runtime::start_rpc_servers::start_rpc_servers,
    sync::{
        applied::{applied_anchor, applied_commit, capture_applied_state},
        follower::ValidatorFollowerSource,
    },
};
use anyhow::{Context, Result};
use eve_node_policy::{ZoneId, development_public_budget};
use eve_sync_client::NativeRpcConfig;
use std::{
    sync::{Arc, atomic::Ordering},
    time::Duration,
};
use tokio::sync::watch;

pub async fn run_development_follower(config: DevelopmentFollowerConfig) -> Result<()> {
    let initialization = config.clone();
    let (owner, reader) =
        tokio::task::spawn_blocking(move || open_development_follower(&initialization)).await??;
    let publication = capture_applied_state(&reader)
        .map_err(|error| anyhow::anyhow!("follower capture: {error:?}"))?;
    let height = applied_commit(&publication).target.height;
    let authenticated = applied_anchor(&publication).is_some();
    let limits = development_public_budget();
    let mempool_limits = MempoolLimits {
        maximum_bytes: usize::try_from(limits.mempool_bytes)
            .context("follower mempool budget overflow")?,
        ..MempoolLimits::default()
    };
    let pool = start_applied_mempool(publication, mempool_limits);
    let rpc_bytes = limits
        .query_cache_bytes
        .checked_add(limits.simulation_overlay_bytes)
        .context("follower RPC budget overflow")?;
    let context = create_applied_rpc_context(
        reader.clone(),
        pool.clone(),
        AppliedRpcConfig {
            state_budget: eve_state::development_state_budget(),
            zone: ZoneId(config.zone_id),
            buffer_bytes: rpc_bytes,
            maximum_simulation_memory_bytes: 8 * 1_048_576,
        },
    )?;
    let (http, ws, http_address, ws_address) = start_rpc_servers(
        RpcListenerAddresses {
            http_address: config.http_address,
            ws_address: config.ws_address,
        },
        Arc::clone(&context),
    )
    .await?;
    let started = serde_json::to_string(&super::super::status_types::RuntimeStarted {
        http_address: http_address.to_string(),
        ws_address: ws_address.to_string(),
        height,
        verification_mode: "AUTHENTICATED_IMPORT_CLASSICAL_DEV",
        authenticated_finality: authenticated,
        zone_id: config.zone_id,
    })?;
    println!("{started}");
    let source = ValidatorFollowerSource {
        rpc: NativeRpcConfig {
            address: config.validator_address,
            timeout: Duration::from_secs(2),
            maximum_request_bytes: 16_384,
            maximum_response_bytes: 262_144,
        },
        maximum_chunk_bytes: 32_768,
    };
    let (shutdown, receiver) = watch::channel(false);
    let mut actor = tokio::spawn(run_follower_actor(
        owner,
        reader,
        pool,
        source,
        Duration::from_millis(config.poll_interval_ms),
        receiver,
    ));
    let result = tokio::select! {
        result = &mut actor => result.context("follower task failed").and_then(|result| result),
        signal = super::wait_for_follower_shutdown_signal::wait_for_follower_shutdown_signal() => {
            let _ = shutdown.send(true);
            let finished = actor.await.context("follower shutdown failed").and_then(|result| result);
            signal.context("follower shutdown signal failed").and(finished)
        }
    };
    context.healthy.store(false, Ordering::Release);
    let _ = http.stop();
    let _ = ws.stop();
    http.stopped().await;
    ws.stopped().await;
    result
}
