use std::{net::SocketAddr, process::Command};

use anyhow::{Context, Result, ensure};

pub fn rpc_json(address: SocketAddr, path: &str) -> Result<serde_json::Value> {
    ensure!(
        address.ip().is_loopback() && path.starts_with('/'),
        "fixture RPC must use loopback"
    );
    let response = Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--fail",
            "--http1.1",
            "--connect-timeout",
            "2",
            "--max-time",
            "8",
            "--max-filesize",
            "1048576",
            "--noproxy",
            "*",
        ])
        .arg(format!("http://{address}{path}"))
        .output()
        .context("run bounded loopback HTTP client")?;
    ensure!(response.status.success(), "fixture RPC HTTP request failed");
    ensure!(
        response.stdout.len() <= 1_048_576,
        "fixture RPC response exceeds limit"
    );
    let record: serde_json::Value = serde_json::from_slice(&response.stdout)?;
    ensure!(record.get("error").is_none(), "RPC error: {record}");
    Ok(record["result"].clone())
}
