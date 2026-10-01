// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use std::{path::Path, process::Command};

pub fn verify_tool_version(
    executable: &Path,
    arguments: &[&str],
    expected: &str,
) -> Result<String> {
    let output = Command::new(executable)
        .args(arguments)
        .output()
        .context("pinned executable must run")?;
    ensure!(
        output.status.success(),
        "pinned executable version command failed"
    );
    ensure!(
        output.stdout.len() <= 4096 && output.stderr.len() <= 4096,
        "unexpected version output size"
    );
    let actual = String::from_utf8(output.stdout)?.trim().to_owned();
    ensure!(
        actual == expected,
        "tool version mismatch: expected {expected:?}, received {actual:?}"
    );
    Ok(actual)
}
