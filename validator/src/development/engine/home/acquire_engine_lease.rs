// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::EngineLease;
use anyhow::{Result, ensure};
use std::{fs::OpenOptions, path::Path};

/// Actual OS lock, not a persistent PID assertion. No stale marker deletion or insecure fallback.
pub(in crate::development::engine) fn acquire_engine_lease(data: &Path) -> Result<EngineLease> {
    let path = data.join(".eve-engine-owner.lock");
    if let Ok(metadata) = std::fs::symlink_metadata(&path) {
        ensure!(
            metadata.is_file() && !metadata.file_type().is_symlink() && metadata.len() == 0,
            "foreign engine owner-lock path"
        );
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(&path)?;
    file.try_lock().map_err(|_| {
        anyhow::anyhow!("task engine namespace already owned or OS lease unavailable")
    })?;
    let lease = EngineLease { file, locked: true };
    lease.file.sync_all()?;
    super::sync_engine_directory(data)?;
    Ok(lease)
}
