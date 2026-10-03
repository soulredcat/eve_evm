// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::process_start;
use anyhow::{Context, Result, ensure};
use rustix::process::{Pid, Signal, kill_process};
use std::{io::ErrorKind, path::Path};
pub(crate) fn signal_owned_process(
    pid: u32,
    signal: &str,
    identity: Option<(&Path, u64)>,
) -> Result<()> {
    if !Path::new(&format!("/proc/{pid}")).exists() {
        return Ok(());
    }
    if let Some((binary, start)) = identity {
        // An engine whose parent was just killed may already be a zombie or reaped:
        // its executable link or stat disappears, and there is nothing left to signal.
        let image = match std::fs::read_link(format!("/proc/{pid}/exe")) {
            Ok(image) => image,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        let started = match process_start(pid) {
            Ok(started) => started,
            Err(error)
                if error
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|error| error.kind() == ErrorKind::NotFound) =>
            {
                return Ok(());
            }
            Err(error) => return Err(error),
        };
        ensure!(
            started == start && image == binary.canonicalize()?,
            "refuse signal after owned child identity changed"
        );
    }
    // Signal directly: minimal hosts and CI containers need not ship a `kill` executable.
    let signal = match signal {
        "TERM" => Signal::TERM,
        "KILL" => Signal::KILL,
        _ => anyhow::bail!("invalid task signal"),
    };
    let target = Pid::from_raw(i32::try_from(pid)?).context("invalid owned child PID")?;
    let delivered = kill_process(target, signal);
    ensure!(
        delivered.is_ok() || !Path::new(&format!("/proc/{pid}")).exists(),
        "owned child signal failed"
    );
    Ok(())
}
