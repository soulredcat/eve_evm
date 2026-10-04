// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::checkpoints::CheckpointAppliedError;
use eve_storage::checkpoints::CheckpointError;
use std::{fs::File, path::Path};

/// The final configured directory must be an owned private directory, never a followed symlink.
pub(super) fn open_checkpoint_recovery_root(path: &Path) -> Result<File, CheckpointAppliedError> {
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{Mode, OFlags, open};
        use std::os::unix::fs::MetadataExt;
        let file = File::from(
            open(
                path,
                OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            )
            .map_err(|error| CheckpointAppliedError::Storage(CheckpointError::Io(error.into())))?,
        );
        let metadata = file
            .metadata()
            .map_err(|error| CheckpointAppliedError::Storage(CheckpointError::Io(error)))?;
        if !metadata.is_dir()
            || metadata.uid() != rustix::process::geteuid().as_raw()
            || metadata.mode() & 0o077 != 0
        {
            return Err(CheckpointAppliedError::Storage(
                CheckpointError::UnsafeEntry,
            ));
        }
        Ok(file)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = path;
        Err(CheckpointAppliedError::Storage(
            CheckpointError::UnsupportedPlatform,
        ))
    }
}
