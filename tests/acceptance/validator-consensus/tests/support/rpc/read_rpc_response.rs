// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MAXIMUM_RPC_BODY_BYTES, MAXIMUM_RPC_HEADER_BYTES};
use anyhow::{Result, ensure};
use std::{net::TcpStream, time::Instant};
pub(crate) fn read_rpc_response(stream: &mut TcpStream, deadline: Instant) -> Result<Vec<u8>> {
    let status = super::read_rpc_line::read_rpc_line(stream, deadline, 256)?;
    let mut words = status.split_ascii_whitespace();
    ensure!(
        matches!(words.next(), Some("HTTP/1.1" | "HTTP/1.0")) && words.next() == Some("200"),
        "native HTTP non-success status"
    );
    let mut header_bytes = status.len() + 2;
    let mut length = None;
    let mut chunked = false;
    loop {
        let line = super::read_rpc_line::read_rpc_line(stream, deadline, MAXIMUM_RPC_HEADER_BYTES)?;
        header_bytes = header_bytes
            .checked_add(line.len() + 2)
            .ok_or_else(|| anyhow::anyhow!("native HTTP header overflow"))?;
        ensure!(
            header_bytes <= MAXIMUM_RPC_HEADER_BYTES,
            "native HTTP header byte limit"
        );
        if line.is_empty() {
            break;
        }
        let (name, value) = line
            .split_once(':')
            .ok_or_else(|| anyhow::anyhow!("invalid native HTTP header"))?;
        let value = value.trim();
        match name.to_ascii_lowercase().as_str() {
            "content-length" => {
                ensure!(
                    length.is_none()
                        && !chunked
                        && value.bytes().all(|byte| byte.is_ascii_digit())
                        && !value.is_empty(),
                    "ambiguous native HTTP length"
                );
                let count: usize = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("native HTTP length overflow"))?;
                ensure!(
                    count <= MAXIMUM_RPC_BODY_BYTES,
                    "native HTTP body byte limit"
                );
                length = Some(count);
            }
            "transfer-encoding" => {
                ensure!(
                    !chunked && length.is_none() && value.eq_ignore_ascii_case("chunked"),
                    "unsupported/ambiguous native HTTP transfer coding"
                );
                chunked = true;
            }
            _ => {}
        }
    }
    if chunked {
        return super::read_chunked_body::read_chunked_body(stream, deadline);
    }
    let mut bytes =
        vec![0; length.ok_or_else(|| anyhow::anyhow!("native HTTP body framing missing"))?];
    super::read_rpc_exact::read_rpc_exact(stream, deadline, &mut bytes)?;
    Ok(bytes)
}
