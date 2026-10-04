// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{REJECTED, STAGING, open_proof_file::open_proof_file, proof_file_name};
use crate::sync::{MasterSyncConfig, types::ArchiveInventory};
use anyhow::{Context, Result, ensure};
use std::fs::File;

/// Incremental bounded inventory; no listing Vec, filename sorting or trust in row count.
/// Exact consecutive membership is checked during canonical recovery.
pub(in crate::sync) fn scan_proof_archive(
    directory: &File,
    config: &MasterSyncConfig,
) -> Result<ArchiveInventory> {
    #[cfg(target_os = "linux")]
    {
        let mut entries = rustix::fs::Dir::read_from(directory)
            .context("MASTER_PROOF_ARCHIVE_DIRECTORY_REOPEN")?;
        let mut inventory = ArchiveInventory {
            completed: 0,
            bytes: 0,
            staging: false,
            rejected: false,
        };
        while let Some(entry) = entries.read() {
            let entry = entry.context("MASTER_PROOF_ARCHIVE_READ_ENTRY")?;
            let bytes = entry.file_name().to_bytes();
            if bytes == b"." || bytes == b".." {
                continue;
            }
            let name = std::str::from_utf8(bytes)?;
            if name == STAGING {
                ensure!(!inventory.staging, "MASTER_DUPLICATE_STAGING");
                inventory.staging = true;
            } else if name == REJECTED {
                ensure!(!inventory.rejected, "MASTER_DUPLICATE_REJECTED_STAGING");
                inventory.rejected = true;
            } else {
                ensure!(
                    bytes.len() == 26
                        && &bytes[20..] == b".proof"
                        && bytes[..20].iter().all(u8::is_ascii_digit),
                    "MASTER_UNEXPLAINED_ARCHIVE_FILE"
                );
                let height: u64 = name[..20].parse()?;
                ensure!(
                    proof_file_name(height)? == name,
                    "MASTER_NONCANONICAL_PROOF_NAME"
                );
                inventory.completed = inventory
                    .completed
                    .checked_add(1)
                    .ok_or_else(|| anyhow::anyhow!("MASTER_ARCHIVE_ARITHMETIC"))?;
                ensure!(
                    inventory.completed <= config.maximum_proof_files,
                    "MASTER_PROOF_COUNT_CAPACITY"
                );
            }
            let file = open_proof_file(directory, name, false)?;
            let length = file.metadata()?.len();
            ensure!(
                length <= u64::try_from(config.maximum_proof_bytes)?,
                "MASTER_PROOF_BYTE_CAPACITY"
            );
            inventory.bytes = inventory
                .bytes
                .checked_add(length)
                .ok_or_else(|| anyhow::anyhow!("MASTER_ARCHIVE_ARITHMETIC"))?;
            ensure!(
                inventory.bytes <= config.maximum_archive_bytes,
                "MASTER_ARCHIVE_BYTE_CAPACITY"
            );
        }
        Ok(inventory)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (directory, config);
        anyhow::bail!("MASTER_ARCHIVE_PLATFORM_REQUIRES_LINUX_DIRECTORY_SYNC");
    }
}
