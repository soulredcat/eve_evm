// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{fs::File, path::Path};

/// Only a contained, non-symlink directory is accepted. Subsequent file operations
/// use this retained descriptor, so path replacement cannot redirect proof I/O.
pub(in crate::sync) fn open_proof_directory(path: &Path) -> Result<File> {
    ensure!(
        std::fs::symlink_metadata(path)?.is_dir(),
        "MASTER_PROOF_DIRECTORY_REQUIRED"
    );
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{Mode, OFlags, open};
        let fd = open(
            path,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )?;
        let directory = File::from(fd);
        directory.sync_all()?;
        Ok(directory)
    }
    #[cfg(not(target_os = "linux"))]
    {
        anyhow::bail!("MASTER_ARCHIVE_PLATFORM_REQUIRES_LINUX_DIRECTORY_SYNC");
    }
}
