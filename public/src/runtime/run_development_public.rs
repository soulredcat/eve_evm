// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::status_types::RuntimeStarted;
use super::{DevelopmentPublicConfig, start_rpc_servers::start_rpc_servers};
use crate::{
    development::{open_development_repository, produce_development_blocks},
    mempool::{MempoolLimits, start_mempool},
    rpc::create_rpc_context,
};
use anyhow::{Context, Result};
use eve_storage::state::{create_state_service, read_state_service, state_reader};
use std::sync::{Arc, atomic::Ordering};
use tokio::sync::watch;
pub async fn run_development_public(config: DevelopmentPublicConfig) -> Result<()> {
    let initialization = config.clone();
    let repository =
        tokio::task::spawn_blocking(move || open_development_repository(&initialization)).await??;
    let service = create_state_service(state_reader(&repository));
    let head = read_state_service(&service)?;
    let height = head.commit().target.height;
    let pool = start_mempool(Arc::new(head.commit().clone()), MempoolLimits::default());
    let context = create_rpc_context(
        &repository,
        pool,
        eve_state::development_state_budget(),
        eve_node_policy::ZoneId(config.zone_id),
    );
    read_state_service(&context.service)?;
    let (http, ws, http_address, ws_address) =
        start_rpc_servers(&config, Arc::clone(&context)).await?;
    let started = serde_json::to_string(&RuntimeStarted {
        http_address: http_address.to_string(),
        ws_address: ws_address.to_string(),
        height,
        verification_mode: "LOCAL_DEV_UNAUTHENTICATED",
        authenticated_finality: false,
        zone_id: config.zone_id,
    })?;
    println!("{started}");
    let (shutdown, receiver) = watch::channel(false);
    let mut producer = tokio::spawn(produce_development_blocks(
        repository,
        Arc::clone(&context),
        config.block_interval_ms,
        receiver,
    ));
    let result = tokio::select! {
        result = &mut producer => result.context("producer task failed").and_then(|result| result),
        result = tokio::signal::ctrl_c() => {
            let signal_result = result.context("development shutdown signal failed");
            let _ = shutdown.send(true);
            let producer_result = producer.await.context("producer shutdown failed").and_then(|result| result);
            signal_result.and(producer_result)
        }
    };
    context.healthy.store(false, Ordering::Release);
    let _ = http.stop();
    let _ = ws.stop();
    http.stopped().await;
    ws.stopped().await;
    result
}
