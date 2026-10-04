// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointLimits, preflight_checkpoint_manifest,
    required_checkpoint_metadata_reservation,
    types::{HEADER_BYTES, MAGIC, REFERENCE_BYTES},
};
use eve_state::{
    StateCommitPreflight, StateVersion, encode_state_version, state_commit_preflight_bytes,
    state_commit_preflight_target_bytes,
};
use sha2::{Digest, Sha256};

pub fn create_checkpoint_manifest(
    body: &StateCommitPreflight<'_>,
    target: &StateVersion,
    limits: &CheckpointLimits,
    reserved_metadata: usize,
) -> Result<Vec<u8>, CheckpointError> {
    if reserved_metadata < required_checkpoint_metadata_reservation(limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let body_bytes = state_commit_preflight_bytes(body);
    if body_bytes.is_empty() || body_bytes.len() > limits.maximum_body_bytes {
        return Err(CheckpointError::InvalidManifest);
    }
    eve_state::preflight_state_commit(body_bytes, &limits.logical)
        .map_err(CheckpointError::State)?;
    let chunks = body_bytes.len().div_ceil(limits.maximum_chunk_bytes);
    if chunks > limits.maximum_chunks {
        return Err(CheckpointError::InvalidManifest);
    }
    let version = encode_state_version(target).map_err(CheckpointError::State)?;
    if version.as_ref() != state_commit_preflight_target_bytes(body) {
        return Err(CheckpointError::TargetMismatch);
    }
    let length = chunks
        .checked_mul(REFERENCE_BYTES)
        .and_then(|bytes| bytes.checked_add(HEADER_BYTES + version.len()))
        .ok_or(CheckpointError::InvalidManifest)?;
    if version.len() > 4_096 || length > limits.maximum_manifest_bytes {
        return Err(CheckpointError::InvalidManifest);
    }
    let mut output = Vec::with_capacity(length);
    output.extend_from_slice(MAGIC);
    output.extend_from_slice(&[0, 0]);
    output.extend_from_slice(
        &u16::try_from(version.len())
            .map_err(|_| CheckpointError::InvalidManifest)?
            .to_be_bytes(),
    );
    output.extend_from_slice(
        &u64::try_from(body_bytes.len())
            .map_err(|_| CheckpointError::InvalidManifest)?
            .to_be_bytes(),
    );
    output.extend_from_slice(
        &u32::try_from(limits.maximum_chunk_bytes)
            .map_err(|_| CheckpointError::InvalidManifest)?
            .to_be_bytes(),
    );
    output.extend_from_slice(
        &u32::try_from(chunks)
            .map_err(|_| CheckpointError::InvalidManifest)?
            .to_be_bytes(),
    );
    output.extend_from_slice(&Sha256::digest(body_bytes));
    output.extend_from_slice(&version);
    for (index, chunk) in body_bytes.chunks(limits.maximum_chunk_bytes).enumerate() {
        output.extend_from_slice(
            &u32::try_from(index)
                .map_err(|_| CheckpointError::InvalidManifest)?
                .to_be_bytes(),
        );
        output.extend_from_slice(
            &u64::try_from(index * limits.maximum_chunk_bytes)
                .map_err(|_| CheckpointError::InvalidManifest)?
                .to_be_bytes(),
        );
        output.extend_from_slice(
            &u32::try_from(chunk.len())
                .map_err(|_| CheckpointError::InvalidManifest)?
                .to_be_bytes(),
        );
        output.extend_from_slice(&Sha256::digest(chunk));
    }
    preflight_checkpoint_manifest(&output, &version, limits)?;
    Ok(output)
}
