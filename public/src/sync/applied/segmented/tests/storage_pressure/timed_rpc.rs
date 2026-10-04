// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use jsonrpsee::{
    core::{client::ClientT, params::ArrayParams},
    http_client::HttpClient,
};
use serde_json::Value;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

/// Serialized JSON-RPC over actual loopback HTTP through the production server/module.
pub(super) async fn timed_rpc(
    client: Arc<HttpClient>,
    method: &'static str,
    params: Vec<Value>,
    timeout_ms: u64,
) -> Result<(Value, u128), String> {
    let started = Instant::now();
    let mut encoded = ArrayParams::new();
    for parameter in params {
        encoded
            .insert(parameter)
            .map_err(|_| "RPC parameter encoding")?;
    }
    let value = tokio::time::timeout(
        Duration::from_millis(timeout_ms),
        client.request::<Value, _>(method, encoded),
    )
    .await
    .map_err(|_| format!("{method}: operation timeout"))?
    .map_err(|_| format!("{method}: HTTP JSON-RPC refusal"))?;
    Ok((value, started.elapsed().as_nanos()))
}
