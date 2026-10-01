// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use std::path::{Component, Path, PathBuf};
pub(crate) fn resolve_data_directory(root: &Path, requested: &Path) -> Result<PathBuf> {
    let root = root.canonicalize().context("development root must exist")?;
    let relative = if requested.is_absolute() {
        requested.strip_prefix(&root)?
    } else {
        requested
    };
    ensure!(
        relative.starts_with("local-tests") && relative != Path::new("local-tests"),
        "development data requires dedicated ignored local-tests descendant"
    );
    let mut resolved = root.clone();
    for component in relative.components() {
        let Component::Normal(segment) = component else {
            anyhow::bail!("data requires ordinary contained path components");
        };
        resolved.push(segment);
        if let Ok(metadata) = std::fs::symlink_metadata(&resolved) {
            ensure!(
                !metadata.file_type().is_symlink() && resolved.canonicalize()?.starts_with(&root),
                "development path escapes root"
            );
        }
    }
    let ignored = std::process::Command::new("git")
        .current_dir(&root)
        .args(["check-ignore", "--quiet", "--"])
        .arg(resolved.join("_public_data_guard"))
        .status()?;
    ensure!(
        ignored.success(),
        "development data must remain Git ignored"
    );
    Ok(resolved)
}
