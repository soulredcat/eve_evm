// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MAXIMUM_RPC_HEADER_BYTES;
use anyhow::{Result, ensure};
use std::{net::TcpStream, time::Instant};
pub(super) fn read_chunked_body(
    stream: &mut TcpStream,
    deadline: Instant,
    maximum: usize,
) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    for _ in 0..65_536 {
        let line = super::read_rpc_line::read_rpc_line(stream, deadline, 128)?;
        let size = line.split(';').next().unwrap_or_default();
        ensure!(
            !size.is_empty()
                && size.len() <= 16
                && size.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "invalid native HTTP chunk size"
        );
        let size = usize::from_str_radix(size, 16)
            .map_err(|_| anyhow::anyhow!("native HTTP chunk size overflow"))?;
        if size == 0 {
            let mut trailer_bytes = 0_usize;
            loop {
                let trailer = super::read_rpc_line::read_rpc_line(
                    stream,
                    deadline,
                    MAXIMUM_RPC_HEADER_BYTES,
                )?;
                trailer_bytes = trailer_bytes
                    .checked_add(trailer.len() + 2)
                    .ok_or_else(|| anyhow::anyhow!("native HTTP trailer overflow"))?;
                ensure!(
                    trailer_bytes <= MAXIMUM_RPC_HEADER_BYTES,
                    "native HTTP trailer byte limit"
                );
                if trailer.is_empty() {
                    return Ok(bytes);
                }
                ensure!(trailer.contains(':'), "invalid native HTTP trailer");
            }
        }
        let length = bytes
            .len()
            .checked_add(size)
            .ok_or_else(|| anyhow::anyhow!("native HTTP body overflow"))?;
        ensure!(length <= maximum, "native HTTP chunk body byte limit");
        let start = bytes.len();
        bytes.resize(length, 0);
        super::read_rpc_exact::read_rpc_exact(stream, deadline, &mut bytes[start..])?;
        let mut suffix = [0; 2];
        super::read_rpc_exact::read_rpc_exact(stream, deadline, &mut suffix)?;
        ensure!(suffix == *b"\r\n", "invalid native HTTP chunk terminator");
    }
    anyhow::bail!("native HTTP chunk count limit")
}
