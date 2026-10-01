// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{io::Write, net::TcpStream, time::Instant};

pub(super) fn write_rpc_bytes(
    stream: &mut TcpStream,
    deadline: Instant,
    bytes: &[u8],
) -> Result<()> {
    let mut position = 0;
    while position < bytes.len() {
        let remaining = deadline.saturating_duration_since(Instant::now());
        ensure!(
            !remaining.is_zero(),
            "native RPC absolute write deadline exceeded"
        );
        stream.set_write_timeout(Some(remaining))?;
        let count = stream.write(&bytes[position..])?;
        ensure!(count > 0, "native RPC request write disconnected");
        position += count;
    }
    Ok(())
}
