// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointError;
use std::fs::File;

pub(super) fn unlink_invalid_checkpoint_entry(
    directory: &File,
    name: &str,
    device: u64,
    inode: u64,
) -> Result<(), CheckpointError> {
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{AtFlags, FileType, statat, unlinkat};
        let entry = statat(directory, name, AtFlags::SYMLINK_NOFOLLOW)
            .map_err(|error| CheckpointError::Io(error.into()))?;
        let kind = FileType::from_raw_mode(entry.st_mode);
        if entry.st_dev != device
            || entry.st_ino != inode
            || !matches!(kind, FileType::RegularFile | FileType::Symlink)
            || kind == FileType::RegularFile && entry.st_nlink != 1
        {
            return Err(CheckpointError::UnsafeEntry);
        }
        unlinkat(directory, name, AtFlags::empty())
            .map_err(|error| CheckpointError::Io(error.into()))?;
        directory.sync_all().map_err(CheckpointError::Io)?;
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (directory, name, device, inode);
        Err(CheckpointError::UnsupportedPlatform)
    }
}
