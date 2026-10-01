// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{io::Read, net::TcpStream, time::Instant};
pub(super) fn read_rpc_bytes(
    stream: &mut TcpStream,
    deadline: Instant,
    bytes: &mut [u8],
) -> Result<usize> {
    let remaining = deadline.saturating_duration_since(Instant::now());
    ensure!(
        !remaining.is_zero(),
        "native RPC absolute deadline exceeded"
    );
    stream.set_read_timeout(Some(remaining))?;
    Ok(stream.read(bytes)?)
}
