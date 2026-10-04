// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofTransfer, checkpoint_proof_file_name::checkpoint_proof_file_name,
    encode_checkpoint_proof_reference::encode_checkpoint_proof_reference,
    read_checkpoint_proof_reference::read_checkpoint_proof_reference,
    seed_checkpoint_proof_stream::seed_checkpoint_proof_stream, types::HEADER_BYTES,
    validate_checkpoint_proof_directory::validate_checkpoint_proof_directory,
    validate_checkpoint_proof_metadata::validate_checkpoint_proof_metadata,
};
use crate::checkpoints::{
    CheckpointError, hash_checkpoint_file::hash_checkpoint_file,
    open_checkpoint_file::open_checkpoint_file,
};
use sha2::Digest;

pub(super) fn validate_checkpoint_proof_witnesses(
    transfer: &CheckpointProofTransfer,
) -> Result<(), CheckpointError> {
    validate_checkpoint_proof_directory(&transfer.directory, transfer.summary.files)?;
    validate_checkpoint_proof_metadata(transfer)?;
    let snapshot = transfer.manifest[48..80].try_into().unwrap();
    let body = transfer.manifest[80..112].try_into().unwrap();
    let target = &transfer.manifest[HEADER_BYTES..HEADER_BYTES + transfer.summary.version_bytes];
    let mut stream = seed_checkpoint_proof_stream(snapshot, body, transfer.summary.height, target);
    for index in 0..transfer.summary.files {
        let reference = read_checkpoint_proof_reference(transfer, index)?;
        let encoded = encode_checkpoint_proof_reference(&reference)?;
        stream.update(&encoded[..24]);
        let mut file = open_checkpoint_file(
            &transfer.directory,
            &checkpoint_proof_file_name(index)?,
            false,
        )?;
        if hash_checkpoint_file(&mut file, reference.length, &mut stream)? != reference.sha256 {
            return Err(CheckpointError::CorruptChunk);
        }
        file.sync_all().map_err(CheckpointError::Io)?;
    }
    if <[u8; 32]>::from(stream.finalize()) != transfer.summary.stream_hash {
        return Err(CheckpointError::CorruptChunk);
    }
    Ok(())
}
