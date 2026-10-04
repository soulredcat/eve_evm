// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointManifestPreflight,
    ensure_uncompleted_checkpoint::ensure_uncompleted_checkpoint,
    is_valid_checkpoint_pending_manifest::is_valid_checkpoint_pending_manifest,
    open_checkpoint_file::open_checkpoint_file,
    open_checkpoint_namespace::open_checkpoint_namespace,
    read_checkpoint_metadata::read_checkpoint_metadata, required_checkpoint_metadata_reservation,
    unlink_invalid_checkpoint_entry::unlink_invalid_checkpoint_entry,
    validate_checkpoint_directory::validate_checkpoint_directory,
    validate_checkpoint_file::validate_checkpoint_file,
};
use std::fs::File;

pub fn repair_invalid_checkpoint_manifest_pending(
    base: &File,
    manifest: &CheckpointManifestPreflight<'_>,
    reserved_metadata: usize,
) -> Result<(), CheckpointError> {
    if reserved_metadata < required_checkpoint_metadata_reservation(&manifest.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let directory = open_checkpoint_namespace(base, &manifest.summary.id, false)?;
    ensure_uncompleted_checkpoint(&directory)?;
    validate_checkpoint_directory(&directory, manifest.summary.chunks)?;
    match open_checkpoint_file(&directory, "manifest.bin", false) {
        Ok(_) => return Err(CheckpointError::NamespaceOccupied),
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{AtFlags, FileType, statat};
        let entry = statat(&directory, "manifest.pending", AtFlags::SYMLINK_NOFOLLOW)
            .map_err(|error| CheckpointError::Io(error.into()))?;
        match FileType::from_raw_mode(entry.st_mode) {
            FileType::Symlink => {}
            FileType::RegularFile if entry.st_nlink == 1 => {
                let length = validate_checkpoint_file(&open_checkpoint_file(
                    &directory,
                    "manifest.pending",
                    false,
                )?)?;
                if length > manifest.limits.maximum_manifest_bytes as u64 {
                    // No read or allocation: a tighter receiver ceiling cannot prove invalidity.
                    return Err(CheckpointError::ValidEntry);
                }
                match read_checkpoint_metadata(
                    &directory,
                    "manifest.pending",
                    manifest.limits.maximum_manifest_bytes,
                ) {
                    Ok(bytes)
                        if bytes == manifest.bytes
                            || is_valid_checkpoint_pending_manifest(&bytes) =>
                    {
                        return Err(CheckpointError::ValidEntry);
                    }
                    Ok(_) | Err(CheckpointError::InvalidManifest) => {}
                    Err(error) => return Err(error),
                }
            }
            _ => return Err(CheckpointError::UnsafeEntry),
        }
        ensure_uncompleted_checkpoint(&directory)?;
        unlink_invalid_checkpoint_entry(&directory, "manifest.pending", entry.st_dev, entry.st_ino)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(CheckpointError::UnsupportedPlatform)
    }
}
