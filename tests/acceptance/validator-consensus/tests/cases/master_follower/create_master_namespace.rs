// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::process::{Command, Stdio};
use std::{fs::Permissions, os::unix::fs::PermissionsExt};

/// The archive requires actual directory sync and atomic no-replace promotion.
/// A Linux-native temporary Git root preserves the contained ignored-data guard;
/// mounted host filesystems may reject the mandatory rename operation.
pub(in crate::cases) fn create_master_namespace() -> Result<tempfile::TempDir> {
    let root = tempfile::Builder::new()
        .prefix("eve-master-cli-")
        .permissions(Permissions::from_mode(0o700))
        .tempdir()?;
    let initialized = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(root.path())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    ensure!(initialized.success(), "MASTER_FOLLOWER_NAMESPACE_GIT_INIT");
    std::fs::write(root.path().join(".gitignore"), "/local-tests/\n")?;
    Ok(root)
}
