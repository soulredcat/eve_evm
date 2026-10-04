// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result};
use eve_sync_client::NativeRpcConfig;
use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

pub(in crate::runtime::follower) fn checkpoint_rpc_before(
    address: SocketAddr,
    deadline: Instant,
) -> Result<NativeRpcConfig> {
    let remaining = deadline
        .checked_duration_since(Instant::now())
        .context("PUBLIC_CHECKPOINT_BOOTSTRAP_DEADLINE")?;
    Ok(NativeRpcConfig {
        address,
        timeout: remaining.min(Duration::from_secs(5)),
        maximum_request_bytes: 262_144,
        maximum_response_bytes: 262_144,
    })
}
