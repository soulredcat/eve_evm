// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointError;
use std::fs::File;

pub(super) fn validate_checkpoint_directory(
    directory: &File,
    chunks: usize,
) -> Result<(), CheckpointError> {
    #[cfg(target_os = "linux")]
    {
        let entries = rustix::fs::Dir::read_from(directory)
            .map_err(|error| CheckpointError::Io(error.into()))?;
        let mut count = 0_usize;
        for entry in entries {
            let entry = entry.map_err(|error| CheckpointError::Io(error.into()))?;
            let name = entry.file_name().to_bytes();
            if matches!(name, b"." | b"..") {
                continue;
            }
            count += 1;
            if count > chunks + 4 {
                return Err(CheckpointError::NamespaceOccupied);
            }
            if matches!(
                name,
                b"manifest.bin" | b"manifest.pending" | b"complete.bin" | b"complete.pending"
            ) {
                continue;
            }
            if name.len() != 14 || &name[..6] != b"chunk-" || &name[10..] != b".bin" {
                return Err(CheckpointError::NamespaceOccupied);
            }
            let mut index = 0_usize;
            for byte in &name[6..10] {
                if !byte.is_ascii_digit() {
                    return Err(CheckpointError::NamespaceOccupied);
                }
                index = index * 10 + usize::from(byte - b'0');
            }
            if index >= chunks {
                return Err(CheckpointError::NamespaceOccupied);
            }
        }
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (directory, chunks);
        Err(CheckpointError::UnsupportedPlatform)
    }
}
