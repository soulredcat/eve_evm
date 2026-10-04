// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointResumeObservation, CheckpointTransfer,
    checkpoint_file_name::checkpoint_file_name, hash_checkpoint_file::hash_checkpoint_file,
    open_checkpoint_file::open_checkpoint_file,
    read_checkpoint_reference::read_checkpoint_reference,
};
use sha2::{Digest, Sha256};

/// The metadata reservation includes the actual fixed 64 KiB scratch for this streaming startup scan.
pub(super) fn scan_checkpoint_resume(
    transfer: &CheckpointTransfer,
) -> Result<CheckpointResumeObservation, CheckpointError> {
    let mut observation = CheckpointResumeObservation::default();
    for index in 0..transfer.summary.chunks {
        let reference = read_checkpoint_reference(transfer, index)?;
        let mut file =
            match open_checkpoint_file(&transfer.directory, &checkpoint_file_name(index)?, false) {
                Ok(file) => file,
                Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                    observation.missing_chunks += 1;
                    continue;
                }
                Err(error) => return Err(error),
            };
        match hash_checkpoint_file(&mut file, reference.length, &mut Sha256::new()) {
            Ok(hash) if hash == reference.hash => observation.verified_chunks += 1,
            Ok(_) | Err(CheckpointError::CorruptChunk) => observation.corrupt_chunks += 1,
            Err(error) => return Err(error),
        }
    }
    Ok(observation)
}
