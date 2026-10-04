// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{io::Write, net::TcpStream, time::Instant};

pub(crate) fn write_rpc_bytes(
    stream: &mut TcpStream,
    deadline: Instant,
    bytes: &[u8],
) -> Result<()> {
    let mut position = 0;
    while position < bytes.len() {
        let remaining = deadline.saturating_duration_since(Instant::now());
        ensure!(!remaining.is_zero(), "B3_RPC_IO_TIMEOUT");
        stream
            .set_write_timeout(Some(remaining))
            .map_err(super::sanitize_rpc_io_error::sanitize_rpc_io_error)?;
        let count = stream
            .write(&bytes[position..])
            .map_err(super::sanitize_rpc_io_error::sanitize_rpc_io_error)?;
        ensure!(count > 0, "B3_RPC_IO_DISCONNECTED");
        position += count;
    }
    Ok(())
}
