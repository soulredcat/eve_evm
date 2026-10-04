// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::rpc::{read_rpc_response, write_rpc_bytes};
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::{
    net::{SocketAddr, TcpStream},
    time::{Duration, Instant},
};

/// Actual bounded POST against the launched product server, using shared framing.
pub(in crate::cases) fn public_rpc(
    address: SocketAddr,
    method: &str,
    params: Value,
) -> Result<Value> {
    ensure!(
        address.ip().is_loopback() && address.port() != 0,
        "PUBLIC_FOLLOWER_RPC_ENDPOINT"
    );
    let deadline = Instant::now() + Duration::from_secs(3);
    let body = serde_json::to_vec(
        &json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params }),
    )?;
    ensure!(body.len() <= 16_384, "PUBLIC_FOLLOWER_RPC_REQUEST_BOUND");
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_secs(1))?;
    let header = format!(
        "POST / HTTP/1.1\r\nHost: {address}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    write_rpc_bytes(&mut stream, deadline, header.as_bytes())?;
    write_rpc_bytes(&mut stream, deadline, &body)?;
    let bytes = read_rpc_response(&mut stream, deadline)?;
    let mut response: Value = serde_json::from_slice(&bytes).context("PUBLIC_FOLLOWER_RPC_JSON")?;
    ensure!(Instant::now() < deadline, "PUBLIC_FOLLOWER_RPC_DEADLINE");
    ensure!(
        response["id"] == 1 && response.get("error").is_none(),
        "PUBLIC_FOLLOWER_RPC_REFUSED"
    );
    response
        .get_mut("result")
        .map(Value::take)
        .context("PUBLIC_FOLLOWER_RPC_RESULT")
}
