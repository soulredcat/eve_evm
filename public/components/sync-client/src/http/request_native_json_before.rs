// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::NativeRpcConfig;
use anyhow::{Result, ensure};
use serde_json::Value;
use std::{net::TcpStream, time::Instant};

/// Preserve the actual caller deadline through formatting, connect, I/O and JSON.
/// The configured timeout may tighten it but can never rebase or extend it.
pub fn request_native_json_before(
    config: NativeRpcConfig,
    path: &str,
    reserved_bytes: usize,
    caller_deadline: Instant,
) -> Result<Value> {
    let configured_deadline = Instant::now()
        .checked_add(config.timeout)
        .ok_or_else(|| anyhow::anyhow!("SYNC_RPC_DEADLINE"))?;
    let deadline = caller_deadline.min(configured_deadline);
    crate::validate_native_rpc_config(config)?;
    let required = crate::required_native_rpc_reservation(config)?;
    ensure!(reserved_bytes >= required, "SYNC_RPC_RESERVATION");
    ensure!(
        path.starts_with('/')
            && path.len() <= config.maximum_request_bytes
            && path
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"/?=&_%+-.~:,".contains(&byte)),
        "SYNC_RPC_REQUEST"
    );
    ensure!(Instant::now() < deadline, "SYNC_RPC_IO_TIMEOUT");
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        config.address
    );
    ensure!(
        request.len() <= config.maximum_request_bytes,
        "SYNC_RPC_REQUEST"
    );
    let remaining = deadline.saturating_duration_since(Instant::now());
    ensure!(!remaining.is_zero(), "SYNC_RPC_IO_TIMEOUT");
    let mut stream = TcpStream::connect_timeout(&config.address, remaining)
        .map_err(super::sanitize_rpc_io_error::sanitize_rpc_io_error)?;
    super::write_rpc_bytes::write_rpc_bytes(&mut stream, deadline, request.as_bytes())?;
    let body = super::read_rpc_response::read_rpc_response(
        &mut stream,
        deadline,
        config.maximum_response_bytes,
    )?;
    let mut value: Value =
        serde_json::from_slice(&body).map_err(|_| anyhow::anyhow!("SYNC_RPC_JSON"))?;
    ensure!(Instant::now() < deadline, "SYNC_RPC_IO_TIMEOUT");
    ensure!(
        value.get("error").is_none_or(Value::is_null),
        "SYNC_RPC_REMOTE_ERROR"
    );
    let result = value
        .get_mut("result")
        .filter(|result| result.is_object())
        .map(Value::take)
        .ok_or_else(|| anyhow::anyhow!("SYNC_RPC_RESULT"))?;
    ensure!(Instant::now() < deadline, "SYNC_RPC_IO_TIMEOUT");
    Ok(result)
}
