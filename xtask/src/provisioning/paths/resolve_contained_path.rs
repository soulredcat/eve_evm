// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use std::path::{Component, Path, PathBuf};

/// Resolve only ordinary relative descendants, rejecting links at every existing prefix.
pub fn resolve_contained_path(base: &Path, relative: &Path) -> Result<PathBuf> {
    ensure!(!relative.as_os_str().is_empty(), "empty artifact path");
    let base = base.canonicalize().context("artifact base must exist")?;
    let mut target = base.clone();
    for component in relative.components() {
        let Component::Normal(segment) = component else {
            anyhow::bail!("artifact path must contain normal relative components only");
        };
        target.push(segment);
        if let Ok(metadata) = std::fs::symlink_metadata(&target) {
            ensure!(
                !metadata.file_type().is_symlink(),
                "artifact path contains a symbolic link"
            );
            ensure!(
                target.canonicalize()?.starts_with(&base),
                "artifact path escaped its base"
            );
        }
    }
    ensure!(target.starts_with(base), "artifact path escaped its base");
    Ok(target)
}
