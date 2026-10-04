// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{STAGING, open_proof_file::open_proof_file, proof_file_name};
use anyhow::{Context, Result};
use std::fs::File;

/// Re-sync and publish without replacement, then sync the actual directory before DB commit.
pub(in crate::sync) fn promote_staged_proof(directory: &File, height: u64) -> Result<()> {
    let staging = open_proof_file(directory, STAGING, false)?;
    let name = proof_file_name(height)?;
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        staging
            .set_permissions(std::fs::Permissions::from_mode(0o400))
            .context("MASTER_STAGING_PERMISSION")?;
        staging.sync_all().context("MASTER_STAGING_SYNC")?;
        rustix::fs::renameat_with(
            directory,
            STAGING,
            directory,
            name.as_str(),
            rustix::fs::RenameFlags::NOREPLACE,
        )
        .context("MASTER_PROOF_PROMOTION_NOREPLACE")?;
        directory
            .sync_all()
            .context("MASTER_PROMOTED_DIRECTORY_SYNC")?;
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = name;
        anyhow::bail!("MASTER_ARCHIVE_PLATFORM_REQUIRES_LINUX_DIRECTORY_SYNC");
    }
}
