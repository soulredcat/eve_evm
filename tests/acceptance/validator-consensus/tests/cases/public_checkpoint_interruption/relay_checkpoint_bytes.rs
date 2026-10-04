// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

pub(super) fn relay_checkpoint_bytes(
    mut input: TcpStream,
    mut output: TcpStream,
    cancelled: &AtomicBool,
    deadline: Instant,
    maximum: usize,
) -> Result<()> {
    let mut buffer = [0_u8; 8_192];
    let mut bytes = 0_usize;
    loop {
        if cancelled.load(Ordering::Acquire) {
            return Ok(());
        }
        ensure!(Instant::now() < deadline, "CHECKPOINT_PROXY_RELAY_DEADLINE");
        let count = match input.read(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(count) => count,
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                continue;
            }
            Err(error) => return Err(error.into()),
        };
        bytes = bytes
            .checked_add(count)
            .ok_or_else(|| anyhow::anyhow!("CHECKPOINT_PROXY_OVERFLOW"))?;
        ensure!(bytes <= maximum, "CHECKPOINT_PROXY_BYTE_LIMIT");
        output.write_all(&buffer[..count])?;
    }
}
