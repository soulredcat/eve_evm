// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofTransfer, checkpoint_proof_file_name::checkpoint_proof_file_name,
    read_checkpoint_proof_reference::read_checkpoint_proof_reference,
};
use crate::checkpoints::{
    CheckpointError, CheckpointResumeObservation, hash_checkpoint_file::hash_checkpoint_file,
    open_checkpoint_file::open_checkpoint_file,
};
use sha2::{Digest, Sha256};

pub(super) fn scan_checkpoint_proof_resume(
    transfer: &CheckpointProofTransfer,
) -> Result<CheckpointResumeObservation, CheckpointError> {
    let mut observation = CheckpointResumeObservation::default();
    for index in 0..transfer.summary.files {
        let reference = read_checkpoint_proof_reference(transfer, index)?;
        let mut file = match open_checkpoint_file(
            &transfer.directory,
            &checkpoint_proof_file_name(index)?,
            false,
        ) {
            Ok(file) => file,
            Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                observation.missing_chunks += 1;
                continue;
            }
            Err(error) => return Err(error),
        };
        match hash_checkpoint_file(&mut file, reference.length, &mut Sha256::new()) {
            Ok(hash) if hash == reference.sha256 => observation.verified_chunks += 1,
            Ok(_) | Err(CheckpointError::CorruptChunk) => observation.corrupt_chunks += 1,
            Err(error) => return Err(error),
        }
    }
    Ok(observation)
}
