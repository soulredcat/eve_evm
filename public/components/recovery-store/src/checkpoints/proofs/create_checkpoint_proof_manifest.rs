// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofLimits, CheckpointProofReferenceInput,
    encode_checkpoint_proof_reference::encode_checkpoint_proof_reference,
    preflight_checkpoint_proof_manifest, required_checkpoint_proof_metadata_reservation,
    types::{HEADER_BYTES, MAGIC, REFERENCE_BYTES},
};
use crate::checkpoints::CheckpointError;
use eve_state::{StateVersion, encode_state_version};

/// Borrowed reference inputs remain separately caller-charged; no raw history bodies are retained.
pub fn create_checkpoint_proof_manifest(
    snapshot_id: &[u8; 32],
    body_sha256: &[u8; 32],
    target: &StateVersion,
    references: &[CheckpointProofReferenceInput],
    stream_sha256: &[u8; 32],
    limits: &CheckpointProofLimits,
    reserved_metadata: usize,
) -> Result<Vec<u8>, CheckpointError> {
    if reserved_metadata < required_checkpoint_proof_metadata_reservation(limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    if target.height >= 10_001
        || references.len() != target.height as usize + 1
        || references.len() > limits.maximum_files
    {
        return Err(CheckpointError::InvalidManifest);
    }
    let version = encode_state_version(target).map_err(CheckpointError::State)?;
    if version.len() > 4_096 {
        return Err(CheckpointError::InvalidManifest);
    }
    let length = references
        .len()
        .checked_mul(REFERENCE_BYTES)
        .and_then(|length| length.checked_add(HEADER_BYTES + version.len()))
        .ok_or(CheckpointError::InvalidManifest)?;
    if length > limits.maximum_manifest_bytes {
        return Err(CheckpointError::InvalidManifest);
    }
    let mut total = 0_usize;
    for reference in references {
        if reference.length == 0 || reference.length > limits.maximum_witness_bytes {
            return Err(CheckpointError::InvalidManifest);
        }
        total = total
            .checked_add(reference.length)
            .ok_or(CheckpointError::ResourceReservation)?;
    }
    let disk = length
        .checked_mul(2)
        .and_then(|bytes| bytes.checked_add(total))
        .and_then(|bytes| bytes.checked_add(160))
        .ok_or(CheckpointError::ResourceReservation)?;
    if total > limits.maximum_total_bytes || disk > limits.maximum_disk_bytes {
        return Err(CheckpointError::ResourceReservation);
    }
    let mut bytes = Vec::with_capacity(length);
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&[0, 0]);
    bytes.extend_from_slice(
        &u16::try_from(version.len())
            .map_err(|_| CheckpointError::InvalidManifest)?
            .to_be_bytes(),
    );
    bytes.extend_from_slice(&target.height.to_be_bytes());
    bytes.extend_from_slice(
        &u32::try_from(references.len())
            .map_err(|_| CheckpointError::InvalidManifest)?
            .to_be_bytes(),
    );
    bytes.extend_from_slice(
        &u64::try_from(total)
            .map_err(|_| CheckpointError::InvalidManifest)?
            .to_be_bytes(),
    );
    bytes.extend_from_slice(snapshot_id);
    bytes.extend_from_slice(body_sha256);
    bytes.extend_from_slice(stream_sha256);
    bytes.extend_from_slice(&version);
    for reference in references {
        bytes.extend_from_slice(&encode_checkpoint_proof_reference(reference)?);
    }
    preflight_checkpoint_proof_manifest(
        &bytes,
        target.height,
        &version,
        snapshot_id,
        body_sha256,
        limits,
    )?;
    Ok(bytes)
}
