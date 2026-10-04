// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, bail};
use std::{path::Path, process::Command};

pub(super) fn require_unignored_adapter_fragment(root: &Path, path: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["check-ignore", "--no-index", "--quiet", "--", path])
        .current_dir(root)
        .output()
        .context("Check adapter fragment ignore policy")?;
    match output.status.code() {
        Some(1) => Ok(()),
        Some(0) => bail!("Ignored adapter policy fragment is forbidden: {path}"),
        _ => bail!(
            "Adapter fragment ignore check failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ),
    }
}
