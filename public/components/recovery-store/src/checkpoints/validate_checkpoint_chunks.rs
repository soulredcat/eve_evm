// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointTransfer, checkpoint_file_name::checkpoint_file_name,
    hash_checkpoint_file::hash_checkpoint_file, open_checkpoint_file::open_checkpoint_file,
    read_checkpoint_reference::read_checkpoint_reference,
    validate_checkpoint_directory::validate_checkpoint_directory,
    validate_checkpoint_metadata::validate_checkpoint_metadata,
};
use sha2::{Digest, Sha256};

pub(super) fn validate_checkpoint_chunks(
    transfer: &CheckpointTransfer,
) -> Result<(), CheckpointError> {
    validate_checkpoint_directory(&transfer.directory, transfer.summary.chunks)?;
    validate_checkpoint_metadata(transfer)?;
    let mut body = Sha256::new();
    for index in 0..transfer.summary.chunks {
        let reference = read_checkpoint_reference(transfer, index)?;
        let name = checkpoint_file_name(index)?;
        let mut file = open_checkpoint_file(&transfer.directory, &name, false)?;
        if hash_checkpoint_file(&mut file, reference.length, &mut body)? != reference.hash {
            return Err(CheckpointError::CorruptChunk);
        }
        file.sync_all().map_err(CheckpointError::Io)?;
    }
    if <[u8; 32]>::from(body.finalize()) != transfer.summary.body_hash {
        return Err(CheckpointError::CorruptChunk);
    }
    Ok(())
}
