// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointProofStreamHasher;
use crate::checkpoints::CheckpointError;
use sha2::Digest;

pub fn finish_checkpoint_proof_stream_hash(
    stream: CheckpointProofStreamHasher,
) -> Result<[u8; 32], CheckpointError> {
    if stream.next != stream.height + 2 {
        return Err(CheckpointError::MissingChunk);
    }
    Ok(stream.hash.finalize().into())
}
