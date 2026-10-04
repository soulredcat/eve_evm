// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{net::TcpStream, time::Instant};
pub(super) fn read_rpc_exact(
    stream: &mut TcpStream,
    deadline: Instant,
    bytes: &mut [u8],
) -> Result<()> {
    let mut position = 0;
    while position < bytes.len() {
        let count =
            super::read_rpc_bytes::read_rpc_bytes(stream, deadline, &mut bytes[position..])?;
        ensure!(count != 0, "SYNC_RPC_IO_DISCONNECTED");
        position += count;
    }
    Ok(())
}
