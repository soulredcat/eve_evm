use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use anyhow::{Context, Result, ensure};

pub fn rpc_json(address: SocketAddr, path: &str) -> Result<serde_json::Value> {
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_secs(2))?;
    stream.set_read_timeout(Some(Duration::from_secs(8)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
    )?;
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes)?;
    let separator = bytes
        .windows(4)
        .position(|bytes| bytes == b"\r\n\r\n")
        .context("RPC response missing header separator")?;
    ensure!(
        bytes.starts_with(b"HTTP/1.1 200"),
        "RPC HTTP error: {}",
        String::from_utf8_lossy(&bytes[..separator])
    );
    let record: serde_json::Value = serde_json::from_slice(&bytes[separator + 4..])?;
    ensure!(record.get("error").is_none(), "RPC error: {record}");
    Ok(record["result"].clone())
}
