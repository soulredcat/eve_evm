// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::Path,
};

/// The already private parent prevents exposure while the repository creates its child.
pub(in crate::consensus::runtime) fn secure_repository_namespace(path: &Path) -> Result<()> {
    let metadata = std::fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_dir() && metadata.uid() == rustix::process::geteuid().as_raw(),
        "foreign repository namespace"
    );
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))?;
    super::sync_node_directory(path)?;
    Ok(())
}
