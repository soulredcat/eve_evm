// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::provisioning::resolve_contained_path;
use anyhow::{Result, ensure};
use std::path::Path;

/// Copy reviewed fixture input into a new ignored package; never copy links or builds.
pub(super) fn copy_client_sources(
    source: &Path,
    destination: &Path,
    total: &mut u64,
) -> Result<()> {
    let metadata = std::fs::symlink_metadata(source)?;
    ensure!(
        metadata.is_dir() && !metadata.file_type().is_symlink(),
        "invalid client source directory"
    );
    std::fs::create_dir_all(destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let child = entry.path();
        let metadata = std::fs::symlink_metadata(&child)?;
        ensure!(
            !metadata.file_type().is_symlink(),
            "client source contains a symbolic link"
        );
        let target = resolve_contained_path(destination, Path::new(&entry.file_name()))?;
        if metadata.is_dir() {
            copy_client_sources(&child, &target, total)?;
        } else {
            ensure!(
                metadata.is_file() && child.extension().is_some_and(|extension| extension == "ts"),
                "unexpected client source artifact"
            );
            *total = total
                .checked_add(metadata.len())
                .ok_or_else(|| anyhow::anyhow!("client byte count overflow"))?;
            ensure!(
                *total <= 5_242_880,
                "client source exceeds reviewed copy budget"
            );
            std::fs::copy(child, target)?;
        }
    }
    Ok(())
}
