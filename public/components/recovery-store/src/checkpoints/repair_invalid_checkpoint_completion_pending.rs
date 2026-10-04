// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointManifestPreflight,
    ensure_uncompleted_checkpoint::ensure_uncompleted_checkpoint,
    is_valid_checkpoint_pending_completion::is_valid_checkpoint_pending_completion,
    open_checkpoint_file::open_checkpoint_file,
    open_checkpoint_namespace::open_checkpoint_namespace,
    read_checkpoint_metadata::read_checkpoint_metadata, required_checkpoint_metadata_reservation,
    unlink_invalid_checkpoint_entry::unlink_invalid_checkpoint_entry,
    validate_checkpoint_directory::validate_checkpoint_directory,
    validate_checkpoint_file::validate_checkpoint_file,
};
use std::fs::File;

/// Remove only a rechecked invalid regular single-link completion staging entry, never valid content.
pub fn repair_invalid_checkpoint_completion_pending(
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
    if read_checkpoint_metadata(&directory, "manifest.bin", manifest.bytes.len())? != manifest.bytes
    {
        return Err(CheckpointError::NamespaceOccupied);
    }
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{AtFlags, FileType, statat};
        let entry = statat(&directory, "complete.pending", AtFlags::SYMLINK_NOFOLLOW)
            .map_err(|error| CheckpointError::Io(error.into()))?;
        if FileType::from_raw_mode(entry.st_mode) != FileType::RegularFile || entry.st_nlink != 1 {
            return Err(CheckpointError::UnsafeEntry);
        }
        let length = validate_checkpoint_file(&open_checkpoint_file(
            &directory,
            "complete.pending",
            false,
        )?)?;
        if length > 80 {
            // Unsupported future framing or capacity is not proof of invalid content.
            return Err(CheckpointError::ValidEntry);
        }
        match read_checkpoint_metadata(&directory, "complete.pending", 80) {
            Ok(bytes) if is_valid_checkpoint_pending_completion(&bytes) => {
                return Err(CheckpointError::ValidEntry);
            }
            Ok(_) => {}
            Err(CheckpointError::InvalidManifest) => return Err(CheckpointError::ValidEntry),
            Err(error) => return Err(error),
        }
        ensure_uncompleted_checkpoint(&directory)?;
        unlink_invalid_checkpoint_entry(&directory, "complete.pending", entry.st_dev, entry.st_ino)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(CheckpointError::UnsupportedPlatform)
    }
}
