// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofTransfer, checkpoint_proof_file_name::checkpoint_proof_file_name,
    observe_checkpoint_proof_witness,
    read_checkpoint_proof_reference::read_checkpoint_proof_reference,
    required_checkpoint_proof_io_reservation,
};
use crate::checkpoints::{
    CheckpointChunkStatus, CheckpointError,
    ensure_uncompleted_checkpoint::ensure_uncompleted_checkpoint,
    open_checkpoint_file::open_checkpoint_file,
};
use sha2::{Digest, Sha256};
use std::io::Write;

pub fn write_checkpoint_proof_witness(
    transfer: &mut CheckpointProofTransfer,
    index: usize,
    bytes: &[u8],
    reserved_io: usize,
) -> Result<(), CheckpointError> {
    if reserved_io < required_checkpoint_proof_io_reservation(&transfer.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let reference = read_checkpoint_proof_reference(transfer, index)?;
    if bytes.len() != reference.length
        || <[u8; 32]>::from(Sha256::digest(bytes)) != reference.sha256
    {
        return Err(CheckpointError::CorruptChunk);
    }
    ensure_uncompleted_checkpoint(&transfer.directory)?;
    let name = checkpoint_proof_file_name(index)?;
    match observe_checkpoint_proof_witness(transfer, index, reserved_io)? {
        CheckpointChunkStatus::Missing => {
            let mut file = open_checkpoint_file(&transfer.directory, &name, true)?;
            file.write_all(bytes).map_err(CheckpointError::Io)?;
            file.sync_all().map_err(CheckpointError::Io)?;
        }
        CheckpointChunkStatus::Present => {}
        CheckpointChunkStatus::Corrupt => return Err(CheckpointError::CorruptChunk),
    }
    let file = open_checkpoint_file(&transfer.directory, &name, false)?;
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o400))
            .map_err(CheckpointError::Io)?;
    }
    file.sync_all().map_err(CheckpointError::Io)?;
    transfer.directory.sync_all().map_err(CheckpointError::Io)?;
    Ok(())
}
