// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::process_start;
use anyhow::{Result, ensure};
use std::{io::ErrorKind, path::Path, process::Command};
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
    ensure!(["TERM", "KILL"].contains(&signal), "invalid task signal");
    let status = Command::new("kill")
        .arg(format!("-{signal}"))
        .arg(pid.to_string())
        .status()?;
    ensure!(
        status.success() || !Path::new(&format!("/proc/{pid}")).exists(),
        "owned child signal failed"
    );
    Ok(())
}
