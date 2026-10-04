// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointError;
use std::fs::File;

pub(super) fn validate_checkpoint_file(file: &File) -> Result<u64, CheckpointError> {
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = file.metadata().map_err(CheckpointError::Io)?;
        if !metadata.is_file() || metadata.nlink() != 1 {
            return Err(CheckpointError::UnsafeEntry);
        }
        Ok(metadata.len())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = file;
        Err(CheckpointError::UnsupportedPlatform)
    }
}
