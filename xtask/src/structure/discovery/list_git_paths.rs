// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, bail};
use std::{collections::BTreeSet, path::Path, process::Command};

pub(super) fn list_git_paths(root: &Path, arguments: &[&str]) -> Result<BTreeSet<String>> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
        .context("Git source discovery could not start")?;
    if !output.status.success() {
        bail!(
            "Git source discovery failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let encoded = String::from_utf8(output.stdout).context("Source paths must be UTF-8")?;
    Ok(encoded
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(str::to_owned)
        .collect())
}
