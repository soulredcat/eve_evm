use anyhow::{Result, ensure};
use std::{path::Path, process::Command};

/// Preserve valid UTF-8 names while escaping controls/backslashes for validation.
pub(crate) fn read_archive_listing(archive: &Path, verbose: bool) -> Result<String> {
    let mut command = Command::new("tar");
    command
        .args(["--list", "--gzip", "--quoting-style=escape"])
        .env("LC_ALL", "C.UTF-8");
    if verbose {
        command.args(["--verbose", "--full-time", "--numeric-owner"]);
    }
    let output = command.arg("--file").arg(archive).output()?;
    ensure!(output.status.success(), "cannot inspect pinned archive");
    Ok(String::from_utf8(output.stdout)?)
}
