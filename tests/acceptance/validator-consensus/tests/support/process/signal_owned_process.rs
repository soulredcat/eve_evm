// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::process_start;
use anyhow::{Result, ensure};
use std::{path::Path, process::Command};
pub(crate) fn signal_owned_process(
    pid: u32,
    signal: &str,
    identity: Option<(&Path, u64)>,
) -> Result<()> {
    if !Path::new(&format!("/proc/{pid}")).exists() {
        return Ok(());
    }
    if let Some((binary, start)) = identity {
        ensure!(
            process_start(pid)? == start
                && std::fs::read_link(format!("/proc/{pid}/exe"))? == binary.canonicalize()?,
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
