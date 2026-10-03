// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::sanitize_rpc_io_error::sanitize_rpc_io_error;
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::{
    net::{SocketAddr, TcpStream},
    time::{Duration, Instant},
};
pub(crate) fn rpc_json(address: SocketAddr, path: &str) -> Result<Value> {
    ensure!(
        address.ip().is_loopback() && address.port() != 0,
        "B3_RPC_ENDPOINT"
    );
    ensure!(
        path.starts_with('/')
            && path.len() <= 300_000
            && path
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"/?=&_%+-.~:,".contains(&byte)),
        "B3_RPC_REQUEST"
    );
    let deadline = Instant::now() + Duration::from_secs(3);
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_secs(1))
        .map_err(sanitize_rpc_io_error)
        .context("B3_RPC_CONNECT")?;
    let request = format!("GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n");
    super::write_rpc_bytes::write_rpc_bytes(&mut stream, deadline, request.as_bytes())
        .context("B3_RPC_WRITE")?;
    let body = super::read_rpc_response::read_rpc_response(&mut stream, deadline)
        .context("B3_RPC_READ")?;
    let mut value: Value =
        serde_json::from_slice(&body).map_err(|_| anyhow::anyhow!("B3_RPC_JSON"))?;
    ensure!(
        value.get("error").is_none_or(Value::is_null),
        "B3_RPC_ERROR"
    );
    value
        .get_mut("result")
        .filter(|result| result.is_object())
        .map(Value::take)
        .ok_or_else(|| anyhow::anyhow!("B3_RPC_RESULT"))
}
