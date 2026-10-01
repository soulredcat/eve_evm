// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::ApplicationListener;
use anyhow::{Result, ensure};
use std::{
    os::unix::{
        fs::{MetadataExt, PermissionsExt},
        net::UnixListener,
    },
    path::Path,
};

pub(in crate::consensus::runtime) fn bind_application_listener(
    path: &Path,
) -> Result<ApplicationListener> {
    ensure!(
        path.as_os_str().len() <= 100 && !path.try_exists()?,
        "application Unix path must be new and bounded"
    );
    let listener = UnixListener::bind(path)?;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    let metadata = std::fs::symlink_metadata(path)?;
    Ok(ApplicationListener {
        listener,
        path: path.to_owned(),
        device: metadata.dev(),
        inode: metadata.ino(),
    })
}
