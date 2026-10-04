// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofWitness, CompletedCheckpointProofStore,
    checkpoint_proof_completion_bytes::checkpoint_proof_completion_bytes,
    checkpoint_proof_file_name::checkpoint_proof_file_name,
    read_checkpoint_proof_reference::read_checkpoint_proof_reference,
    required_checkpoint_proof_witness_reservation,
    validate_checkpoint_proof_directory::validate_checkpoint_proof_directory,
    validate_checkpoint_proof_metadata::validate_checkpoint_proof_metadata,
};
use crate::checkpoints::{
    CheckpointError, open_checkpoint_file::open_checkpoint_file,
    read_checkpoint_metadata::read_checkpoint_metadata,
    validate_checkpoint_file::validate_checkpoint_file,
};
use sha2::{Digest, Sha256};
use std::io::Read;

pub fn read_checkpoint_proof_witness(
    store: &CompletedCheckpointProofStore,
    index: usize,
    reserved_body: usize,
) -> Result<CheckpointProofWitness, CheckpointError> {
    if reserved_body < required_checkpoint_proof_witness_reservation(store, index)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let transfer = &store.transfer;
    validate_checkpoint_proof_directory(&transfer.directory, transfer.summary.files)?;
    validate_checkpoint_proof_metadata(transfer)?;
    if read_checkpoint_metadata(&transfer.directory, "complete.bin", 80)?
        != checkpoint_proof_completion_bytes(&transfer.summary)
    {
        return Err(CheckpointError::NamespaceOccupied);
    }
    let reference = read_checkpoint_proof_reference(transfer, index)?;
    let mut file = open_checkpoint_file(
        &transfer.directory,
        &checkpoint_proof_file_name(index)?,
        false,
    )?;
    if usize::try_from(validate_checkpoint_file(&file)?).ok() != Some(reference.length) {
        return Err(CheckpointError::CorruptChunk);
    }
    let mut bytes = vec![0_u8; reference.length];
    file.read_exact(&mut bytes).map_err(CheckpointError::Io)?;
    let mut extra = [0_u8; 1];
    if file.read(&mut extra).map_err(CheckpointError::Io)? != 0
        || <[u8; 32]>::from(Sha256::digest(&bytes)) != reference.sha256
        || usize::try_from(validate_checkpoint_file(&file)?).ok() != Some(reference.length)
    {
        return Err(CheckpointError::CorruptChunk);
    }
    Ok(CheckpointProofWitness { bytes, reference })
}
