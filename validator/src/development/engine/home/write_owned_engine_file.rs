// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{fs::OpenOptions, io::Write, path::Path};

/// Atomic replace only within a previously validated task-owned engine namespace.
pub(in crate::development::engine) fn write_owned_engine_file(
    path: &Path,
    bytes: &[u8],
    maximum: usize,
) -> Result<()> {
    ensure!(bytes.len() <= maximum, "engine owned-file byte limit");
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("engine file parent missing"))?;
    let prefix = super::engine_artifact_prefix("write")?;
    let temporary = parent.join(format!(".eve-{prefix}"));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    std::fs::rename(&temporary, path)?;
    super::sync_engine_directory(parent)
}
