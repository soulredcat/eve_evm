// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::cases::public_follower::PublicFollower;
use anyhow::{Result, ensure};
use std::{os::unix::fs::MetadataExt, path::Path};

/// Only metadata is observed; the preserved copy never supplies recovery bytes.
pub(super) fn verify_preserved_replica(replica: &PublicFollower, lost: &Path) -> Result<()> {
    let current = std::fs::symlink_metadata(&replica.data)?;
    let preserved = std::fs::symlink_metadata(lost)?;
    ensure!(
        current.is_dir()
            && !current.file_type().is_symlink()
            && preserved.is_dir()
            && !preserved.file_type().is_symlink()
            && (current.dev(), current.ino()) != (preserved.dev(), preserved.ino()),
        "PUBLIC_REPLICA_RECOVERY_REUSED_THE_PRESERVED_DIRECTORY"
    );
    Ok(())
}
