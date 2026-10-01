// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::cli::types::{DevelopmentOptions, ListenerOptions};
use anyhow::Result;
use eve_public::runtime::{DevelopmentPublicConfig, run_development_public};

/// Compose public development services without duplicating role-private behavior.
pub fn serve_development_rpc(
    options: DevelopmentOptions,
    listeners: ListenerOptions,
) -> Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?
        .block_on(run_development_public(DevelopmentPublicConfig {
            root: options.root,
            data: options.data,
            genesis: options.genesis,
            mode: options.mode,
            acknowledge_unsafe_development: options.acknowledge_unsafe_development,
            http_address: listeners.http_address,
            ws_address: listeners.ws_address,
            block_interval_ms: listeners.block_interval_ms,
            allow_external_bind: listeners.allow_external_bind,
            zone_id: listeners.zone_id,
        }))
}
