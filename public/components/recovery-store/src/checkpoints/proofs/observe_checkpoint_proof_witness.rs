// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofTransfer, checkpoint_proof_file_name::checkpoint_proof_file_name,
    read_checkpoint_proof_reference::read_checkpoint_proof_reference,
    required_checkpoint_proof_io_reservation,
};
use crate::checkpoints::{
    CheckpointChunkStatus, CheckpointError, hash_checkpoint_file::hash_checkpoint_file,
    open_checkpoint_file::open_checkpoint_file,
};
use sha2::{Digest, Sha256};

pub fn observe_checkpoint_proof_witness(
    transfer: &CheckpointProofTransfer,
    index: usize,
    reserved_io: usize,
) -> Result<CheckpointChunkStatus, CheckpointError> {
    if reserved_io < required_checkpoint_proof_io_reservation(&transfer.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let reference = read_checkpoint_proof_reference(transfer, index)?;
    let mut file = match open_checkpoint_file(
        &transfer.directory,
        &checkpoint_proof_file_name(index)?,
        false,
    ) {
        Ok(file) => file,
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(CheckpointChunkStatus::Missing);
        }
        Err(error) => return Err(error),
    };
    match hash_checkpoint_file(&mut file, reference.length, &mut Sha256::new()) {
        Ok(hash) if hash == reference.sha256 => Ok(CheckpointChunkStatus::Present),
        Ok(_) | Err(CheckpointError::CorruptChunk) => Ok(CheckpointChunkStatus::Corrupt),
        Err(error) => Err(error),
    }
}
