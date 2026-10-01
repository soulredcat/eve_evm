// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::ApplicationListener;
use anyhow::{Result, ensure};
use std::os::unix::fs::{FileTypeExt, MetadataExt};

pub(in crate::consensus::runtime) fn cleanup_application_listener(
    listener: &ApplicationListener,
) -> Result<()> {
    match std::fs::symlink_metadata(&listener.path) {
        Ok(metadata) => {
            ensure!(
                metadata.file_type().is_socket()
                    && metadata.dev() == listener.device
                    && metadata.ino() == listener.inode
                    && metadata.uid() == rustix::process::geteuid().as_raw(),
                "application endpoint identity changed; preserve foreign path"
            );
            std::fs::remove_file(&listener.path)?;
            crate::consensus::runtime::namespace::sync_node_directory(
                listener
                    .path
                    .parent()
                    .ok_or_else(|| anyhow::anyhow!("application endpoint parent missing"))?,
            )?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    Ok(())
}
