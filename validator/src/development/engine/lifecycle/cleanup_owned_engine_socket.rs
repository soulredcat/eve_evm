// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{io::ErrorKind, path::Path};

/// Only the designated ephemeral endpoint, absent before our launch, may be removed after child death.
pub(in crate::development::engine) fn cleanup_owned_engine_socket(
    data: &Path,
    socket: &Path,
) -> Result<()> {
    let parent = std::fs::symlink_metadata(data)?;
    ensure!(
        parent.is_dir() && !parent.file_type().is_symlink(),
        "owned engine socket parent must remain a real directory"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        ensure!(
            parent.uid() == rustix::process::geteuid().as_raw() && parent.mode() & 0o777 == 0o700,
            "owned engine socket cleanup requires actual private current-UID parent"
        );
    }
    ensure!(
        socket
            .parent()
            .ok_or_else(|| anyhow::anyhow!("owned engine socket parent missing"))?
            .canonicalize()?
            == data.canonicalize()?,
        "owned engine socket escaped task data"
    );
    let metadata = match std::fs::symlink_metadata(socket) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    #[cfg(unix)]
    {
        use std::os::unix::fs::{FileTypeExt, MetadataExt};
        ensure!(
            metadata.file_type().is_socket()
                && metadata.uid() == rustix::process::geteuid().as_raw(),
            "foreign engine socket path preserved"
        );
    }
    std::fs::remove_file(socket)?;
    crate::development::engine::home::sync_engine_directory(data)
}
