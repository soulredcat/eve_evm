// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofManifestPreflight, CheckpointProofPendingKind,
    is_valid_checkpoint_proof_pending::is_valid_checkpoint_proof_pending,
    required_checkpoint_proof_metadata_reservation,
    validate_checkpoint_proof_directory::validate_checkpoint_proof_directory,
};
use crate::checkpoints::{
    CheckpointError, ensure_uncompleted_checkpoint::ensure_uncompleted_checkpoint,
    open_checkpoint_file::open_checkpoint_file,
    open_checkpoint_namespace::open_checkpoint_namespace,
    read_checkpoint_metadata::read_checkpoint_metadata,
    unlink_invalid_checkpoint_entry::unlink_invalid_checkpoint_entry,
    validate_checkpoint_file::validate_checkpoint_file,
};
use std::fs::File;

pub fn repair_invalid_checkpoint_proof_pending(
    base: &File,
    manifest: &CheckpointProofManifestPreflight<'_>,
    kind: CheckpointProofPendingKind,
    reserved_metadata: usize,
) -> Result<(), CheckpointError> {
    if reserved_metadata < required_checkpoint_proof_metadata_reservation(&manifest.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let directory = open_checkpoint_namespace(base, &manifest.summary.id, false)?;
    ensure_uncompleted_checkpoint(&directory)?;
    validate_checkpoint_proof_directory(&directory, manifest.summary.files)?;
    let (name, maximum) = match kind {
        CheckpointProofPendingKind::Manifest => {
            match open_checkpoint_file(&directory, "manifest.bin", false) {
                Ok(_) => return Err(CheckpointError::NamespaceOccupied),
                Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                }
                Err(error) => return Err(error),
            }
            ("manifest.pending", manifest.limits.maximum_manifest_bytes)
        }
        CheckpointProofPendingKind::Completion => {
            if read_checkpoint_metadata(&directory, "manifest.bin", manifest.bytes.len())?
                != manifest.bytes
            {
                return Err(CheckpointError::NamespaceOccupied);
            }
            ("complete.pending", 80)
        }
    };
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{AtFlags, FileType, statat};
        let entry = statat(&directory, name, AtFlags::SYMLINK_NOFOLLOW)
            .map_err(|error| CheckpointError::Io(error.into()))?;
        if FileType::from_raw_mode(entry.st_mode) != FileType::RegularFile || entry.st_nlink != 1 {
            return Err(CheckpointError::UnsafeEntry);
        }
        let length = validate_checkpoint_file(&open_checkpoint_file(&directory, name, false)?)?;
        if length > maximum as u64 {
            // Refuse before owned read: exceeding current admission cannot prove invalidity.
            return Err(CheckpointError::ValidEntry);
        }
        match read_checkpoint_metadata(&directory, name, maximum) {
            Ok(bytes) if is_valid_checkpoint_proof_pending(&bytes, kind) => {
                return Err(CheckpointError::ValidEntry);
            }
            Ok(_) | Err(CheckpointError::InvalidManifest) => {}
            Err(error) => return Err(error),
        }
        ensure_uncompleted_checkpoint(&directory)?;
        unlink_invalid_checkpoint_entry(&directory, name, entry.st_dev, entry.st_ino)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(CheckpointError::UnsupportedPlatform)
    }
}
