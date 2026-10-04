// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{net::TcpStream, time::Instant};
pub(super) fn read_rpc_line(
    stream: &mut TcpStream,
    deadline: Instant,
    maximum: usize,
) -> Result<String> {
    let mut bytes = Vec::new();
    loop {
        ensure!(bytes.len() < maximum, "native HTTP line byte limit");
        let mut byte = [0];
        super::read_rpc_exact::read_rpc_exact(stream, deadline, &mut byte)?;
        bytes.push(byte[0]);
        if byte[0] == b'\n' {
            ensure!(bytes.ends_with(b"\r\n"), "native HTTP requires CRLF");
            bytes.truncate(bytes.len() - 2);
            return String::from_utf8(bytes)
                .map_err(|_| anyhow::anyhow!("native HTTP line encoding"));
        }
    }
}
