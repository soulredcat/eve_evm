// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::resolve_contained_path;
use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};

pub fn prepare_output_directory(root: &Path, output: &Path) -> Result<PathBuf> {
    let root = root.canonicalize().context("repository root must exist")?;
    let relative = if output.is_absolute() {
        output
            .strip_prefix(&root)
            .context("output must be beneath the repository")?
    } else {
        output
    };
    ensure!(
        relative.starts_with("local-tests") && relative != Path::new("local-tests"),
        "tools must be inside a dedicated ignored local-tests descendant"
    );
    let target = resolve_contained_path(&root, relative)?;
    std::fs::create_dir_all(&target)?;
    resolve_contained_path(&root, relative)
}
