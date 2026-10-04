// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofTransfer, checkpoint_proof_file_name::checkpoint_proof_file_name,
    observe_checkpoint_proof_witness, required_checkpoint_proof_io_reservation,
    validate_checkpoint_proof_directory::validate_checkpoint_proof_directory,
};
use crate::checkpoints::{
    CheckpointChunkStatus, CheckpointError,
    ensure_uncompleted_checkpoint::ensure_uncompleted_checkpoint,
    unlink_invalid_checkpoint_entry::unlink_invalid_checkpoint_entry,
};

pub fn repair_invalid_checkpoint_proof_witness(
    transfer: &mut CheckpointProofTransfer,
    index: usize,
    reserved_io: usize,
) -> Result<(), CheckpointError> {
    if reserved_io < required_checkpoint_proof_io_reservation(&transfer.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    if index >= transfer.summary.files {
        return Err(CheckpointError::InvalidManifest);
    }
    ensure_uncompleted_checkpoint(&transfer.directory)?;
    validate_checkpoint_proof_directory(&transfer.directory, transfer.summary.files)?;
    let name = checkpoint_proof_file_name(index)?;
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{AtFlags, FileType, statat};
        let entry = statat(
            &transfer.directory,
            name.as_str(),
            AtFlags::SYMLINK_NOFOLLOW,
        )
        .map_err(|error| CheckpointError::Io(error.into()))?;
        match FileType::from_raw_mode(entry.st_mode) {
            FileType::Symlink => {}
            FileType::RegularFile if entry.st_nlink == 1 => {
                match observe_checkpoint_proof_witness(transfer, index, reserved_io)? {
                    CheckpointChunkStatus::Corrupt => {}
                    CheckpointChunkStatus::Present => return Err(CheckpointError::ValidEntry),
                    CheckpointChunkStatus::Missing => return Err(CheckpointError::MissingChunk),
                }
            }
            _ => return Err(CheckpointError::UnsafeEntry),
        }
        ensure_uncompleted_checkpoint(&transfer.directory)?;
        unlink_invalid_checkpoint_entry(&transfer.directory, &name, entry.st_dev, entry.st_ino)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(CheckpointError::UnsupportedPlatform)
    }
}
