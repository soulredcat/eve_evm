// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{REJECTED, STAGING, open_proof_file::open_proof_file};
use anyhow::Result;
use std::fs::File;

/// Preserve malformed uncommitted input in exactly one bounded slot. An occupied
/// slot refuses reconciliation; completed native proof files are never removed.
pub(in crate::sync) fn reject_staged_proof(directory: &File) -> Result<()> {
    let staging = open_proof_file(directory, STAGING, false)?;
    staging.sync_all()?;
    #[cfg(target_os = "linux")]
    {
        rustix::fs::renameat_with(
            directory,
            STAGING,
            directory,
            REJECTED,
            rustix::fs::RenameFlags::NOREPLACE,
        )?;
        directory.sync_all()?;
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        anyhow::bail!("MASTER_ARCHIVE_PLATFORM_REQUIRES_LINUX_DIRECTORY_SYNC");
    }
}
