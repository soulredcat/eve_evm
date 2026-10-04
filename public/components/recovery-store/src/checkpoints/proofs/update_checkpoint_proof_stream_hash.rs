// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofKind, CheckpointProofReferenceInput, CheckpointProofStreamHasher,
    encode_checkpoint_proof_reference::encode_checkpoint_proof_reference,
};
use crate::checkpoints::CheckpointError;
use sha2::{Digest, Sha256};

pub fn update_checkpoint_proof_stream_hash(
    stream: &mut CheckpointProofStreamHasher,
    reference: &CheckpointProofReferenceInput,
    bytes: &[u8],
) -> Result<(), CheckpointError> {
    let kind = if stream.next <= stream.height {
        CheckpointProofKind::Execution
    } else {
        CheckpointProofKind::ClosingLookahead
    };
    if stream.next > stream.height + 1
        || reference.height != stream.next
        || reference.kind != kind
        || reference.length == 0
        || reference.length > stream.limits.maximum_witness_bytes
        || bytes.len() != reference.length
    {
        return Err(CheckpointError::InvalidManifest);
    }
    let total = stream
        .total
        .checked_add(bytes.len())
        .ok_or(CheckpointError::ResourceReservation)?;
    if total > stream.limits.maximum_total_bytes {
        return Err(CheckpointError::ResourceReservation);
    }
    if <[u8; 32]>::from(Sha256::digest(bytes)) != reference.sha256 {
        return Err(CheckpointError::CorruptChunk);
    }
    let encoded = encode_checkpoint_proof_reference(reference)?;
    stream.hash.update(&encoded[..24]);
    stream.hash.update(bytes);
    stream.total = total;
    stream.next += 1;
    Ok(())
}
