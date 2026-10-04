// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofLimits, CheckpointProofStreamHasher,
    required_checkpoint_proof_metadata_reservation,
    seed_checkpoint_proof_stream::seed_checkpoint_proof_stream,
};
use crate::checkpoints::CheckpointError;
use eve_state::{StateVersion, encode_state_version};

pub fn begin_checkpoint_proof_stream_hash(
    snapshot_id: &[u8; 32],
    body_sha256: &[u8; 32],
    target: &StateVersion,
    limits: &CheckpointProofLimits,
    reserved_metadata: usize,
) -> Result<CheckpointProofStreamHasher, CheckpointError> {
    if reserved_metadata < required_checkpoint_proof_metadata_reservation(limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    if target.height >= 10_001 || target.height + 1 > limits.maximum_files as u64 {
        return Err(CheckpointError::InvalidManifest);
    }
    let version = encode_state_version(target).map_err(CheckpointError::State)?;
    if version.len() > 4_096 {
        return Err(CheckpointError::InvalidManifest);
    }
    Ok(CheckpointProofStreamHasher {
        hash: seed_checkpoint_proof_stream(snapshot_id, body_sha256, target.height, &version),
        height: target.height,
        next: 1,
        limits: *limits,
        total: 0,
    })
}
