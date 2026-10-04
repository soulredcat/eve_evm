// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointChunkStatus, CheckpointError, CheckpointTransfer,
    checkpoint_file_name::checkpoint_file_name, hash_checkpoint_file::hash_checkpoint_file,
    open_checkpoint_file::open_checkpoint_file,
    read_checkpoint_reference::read_checkpoint_reference, required_checkpoint_io_reservation,
};
use sha2::{Digest, Sha256};

pub fn observe_checkpoint_chunk(
    transfer: &CheckpointTransfer,
    index: usize,
    reserved_io: usize,
) -> Result<CheckpointChunkStatus, CheckpointError> {
    if reserved_io < required_checkpoint_io_reservation(&transfer.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let reference = read_checkpoint_reference(transfer, index)?;
    let name = checkpoint_file_name(index)?;
    let mut file = match open_checkpoint_file(&transfer.directory, &name, false) {
        Ok(file) => file,
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(CheckpointChunkStatus::Missing);
        }
        Err(error) => return Err(error),
    };
    match hash_checkpoint_file(&mut file, reference.length, &mut Sha256::new()) {
        Ok(hash) if hash == reference.hash => Ok(CheckpointChunkStatus::Present),
        Ok(_) | Err(CheckpointError::CorruptChunk) => Ok(CheckpointChunkStatus::Corrupt),
        Err(error) => Err(error),
    }
}
