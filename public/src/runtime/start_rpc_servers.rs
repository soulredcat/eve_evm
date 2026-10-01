// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::DevelopmentPublicConfig;
use crate::rpc::{RpcContext, create_rpc_module};
use anyhow::Result;
use jsonrpsee::core::id_providers::RandomStringIdProvider;
use jsonrpsee::server::{BatchRequestConfig, ServerBuilder, ServerConfig, ServerHandle};
use std::{net::SocketAddr, sync::Arc};
pub(crate) async fn start_rpc_servers(
    config: &DevelopmentPublicConfig,
    context: Arc<RpcContext>,
) -> Result<(ServerHandle, ServerHandle, SocketAddr, SocketAddr)> {
    let limits = || {
        ServerConfig::builder()
            .max_request_body_size(1_048_576)
            .max_response_body_size(4 * 1_048_576)
            .set_batch_request_config(BatchRequestConfig::Limit(100))
            .max_subscriptions_per_connection(16)
            .set_message_buffer_capacity(32)
            .set_id_provider(RandomStringIdProvider::new(16))
    };
    let http = ServerBuilder::with_config(limits().http_only().max_connections(128).build())
        .build(config.http_address)
        .await?;
    let ws = ServerBuilder::with_config(
        limits()
            .ws_only()
            .max_request_body_size(65_536)
            .max_response_body_size(65_536)
            .max_connections(64)
            .build(),
    )
    .build(config.ws_address)
    .await?;
    let http_address = http.local_addr()?;
    let ws_address = ws.local_addr()?;
    let module = create_rpc_module(context)?;
    Ok((
        http.start(module.clone()),
        ws.start(module),
        http_address,
        ws_address,
    ))
}
