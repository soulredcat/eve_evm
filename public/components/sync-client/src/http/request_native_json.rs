// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::NativeRpcConfig;
use anyhow::{Result, ensure};
use serde_json::Value;
use std::{net::TcpStream, time::Instant};

/// Bounded loopback development transport. The caller leases request/response and
/// JSON materialization capacity before calling; this function grants no authority.
pub fn request_native_json(
    config: NativeRpcConfig,
    path: &str,
    reserved_bytes: usize,
) -> Result<Value> {
    crate::validate_native_rpc_config(config)?;
    let required = super::super::required_native_rpc_reservation(config)?;
    ensure!(reserved_bytes >= required, "SYNC_RPC_RESERVATION");
    ensure!(
        path.starts_with('/')
            && path.len() <= config.maximum_request_bytes
            && path
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"/?=&_%+-.~:,".contains(&byte)),
        "SYNC_RPC_REQUEST"
    );
    let deadline = Instant::now()
        .checked_add(config.timeout)
        .ok_or_else(|| anyhow::anyhow!("SYNC_RPC_DEADLINE"))?;

    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        config.address
    );
    ensure!(
        request.len() <= config.maximum_request_bytes,
        "SYNC_RPC_REQUEST"
    );
    let mut stream = TcpStream::connect_timeout(&config.address, config.timeout)
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
    value
        .get_mut("result")
        .filter(|result| result.is_object())
        .map(Value::take)
        .ok_or_else(|| anyhow::anyhow!("SYNC_RPC_RESULT"))
}
