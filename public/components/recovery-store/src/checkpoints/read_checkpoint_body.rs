// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointBody, CheckpointError, CompletedCheckpointStore,
    checkpoint_completion_bytes::checkpoint_completion_bytes,
    checkpoint_file_name::checkpoint_file_name, open_checkpoint_file::open_checkpoint_file,
    read_checkpoint_metadata::read_checkpoint_metadata,
    read_checkpoint_reference::read_checkpoint_reference, required_checkpoint_body_reservation,
    validate_checkpoint_directory::validate_checkpoint_directory,
    validate_checkpoint_file::validate_checkpoint_file,
    validate_checkpoint_metadata::validate_checkpoint_metadata,
};
use sha2::{Digest, Sha256};
use std::io::Read;

pub fn read_checkpoint_body(
    store: &CompletedCheckpointStore,
    reserved_body: usize,
) -> Result<CheckpointBody, CheckpointError> {
    if reserved_body < required_checkpoint_body_reservation(store)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let transfer = &store.transfer;
    validate_checkpoint_directory(&transfer.directory, transfer.summary.chunks)?;
    validate_checkpoint_metadata(transfer)?;
    if read_checkpoint_metadata(&transfer.directory, "complete.bin", 80)?
        != checkpoint_completion_bytes(&transfer.summary)
    {
        return Err(CheckpointError::NamespaceOccupied);
    }
    let mut bytes = vec![0_u8; transfer.summary.body_bytes];
    for index in 0..transfer.summary.chunks {
        let reference = read_checkpoint_reference(transfer, index)?;
        let mut file =
            open_checkpoint_file(&transfer.directory, &checkpoint_file_name(index)?, false)?;
        if usize::try_from(validate_checkpoint_file(&file)?).ok() != Some(reference.length) {
            return Err(CheckpointError::CorruptChunk);
        }
        let chunk = &mut bytes[reference.offset..reference.offset + reference.length];
        file.read_exact(chunk).map_err(CheckpointError::Io)?;
        let mut extra = [0_u8; 1];
        if file.read(&mut extra).map_err(CheckpointError::Io)? != 0
            || <[u8; 32]>::from(Sha256::digest(&*chunk)) != reference.hash
            || usize::try_from(validate_checkpoint_file(&file)?).ok() != Some(reference.length)
        {
            return Err(CheckpointError::CorruptChunk);
        }
    }
    if <[u8; 32]>::from(Sha256::digest(&bytes)) != transfer.summary.body_hash {
        return Err(CheckpointError::CorruptChunk);
    }
    let preflight = eve_state::preflight_state_commit(&bytes, &transfer.limits.logical)
        .map_err(CheckpointError::State)?;
    if eve_state::state_commit_preflight_target_bytes(&preflight)
        != super::checkpoint_target_encoding(store)
    {
        return Err(CheckpointError::TargetMismatch);
    }
    Ok(CheckpointBody {
        bytes,
        budget: transfer.limits.logical,
    })
}
