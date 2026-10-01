// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::structure::policy::validate_relative_path::validate_relative_path;
use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

pub fn resolve_source_path(root: &Path, path: &str) -> Result<PathBuf> {
    validate_relative_path(path)?;
    let root = root.canonicalize()?;
    let mut current = root.clone();
    for segment in path.split('/') {
        current.push(segment);
        let metadata = std::fs::symlink_metadata(&current)?;
        if metadata.file_type().is_symlink() {
            bail!("Linked source component is forbidden: {path}");
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            if metadata.file_attributes() & 0x400 != 0 {
                bail!("Reparse-point source component is forbidden: {path}");
            }
        }
    }
    let canonical = current.canonicalize()?;
    if !canonical.starts_with(&root) || !canonical.is_file() {
        bail!("Source escapes the repository or is not a file: {path}");
    }
    Ok(canonical)
}
