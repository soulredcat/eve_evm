// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointBaseError, CheckpointBaseLimits, CheckpointBaseMetadata,
    PreparedCheckpointBaseTarget, required_checkpoint_base_encoding_reservation,
    types::{DOMAIN, HEADER_BYTES},
    validate_checkpoint_base_metadata::validate_checkpoint_base_metadata,
};
use sha2::{Digest, Sha256};

/// Canonical local base encoding. Referenced content, real WAL acknowledgement
/// and authenticated checkpoint activation must be checked separately.
pub fn encode_checkpoint_base(
    metadata: CheckpointBaseMetadata,
    target: &PreparedCheckpointBaseTarget,
    limits: &CheckpointBaseLimits,
    reserved_bytes: usize,
) -> Result<Vec<u8>, CheckpointBaseError> {
    if reserved_bytes < required_checkpoint_base_encoding_reservation(limits)? {
        return Err(CheckpointBaseError::InsufficientReservation);
    }
    validate_checkpoint_base_metadata(&metadata, target)?;
    let length = HEADER_BYTES
        .checked_add(target.encoded.len())
        .and_then(|bytes| bytes.checked_add(32))
        .ok_or(CheckpointBaseError::ArithmeticOverflow)?;
    if length > limits.maximum_payload_bytes {
        return Err(CheckpointBaseError::LimitExceeded);
    }
    let version_length =
        u16::try_from(target.encoded.len()).map_err(|_| CheckpointBaseError::LimitExceeded)?;
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(length)
        .map_err(|_| CheckpointBaseError::AllocationFailed)?;
    bytes.extend_from_slice(DOMAIN);
    bytes.extend_from_slice(&1_u16.to_be_bytes());
    bytes.push(metadata.mode as u8);
    bytes.push(target.security_profile);
    bytes.extend_from_slice(&version_length.to_be_bytes());
    bytes.extend_from_slice(&metadata.previous_opaque_cursor.sequence.to_be_bytes());
    bytes.extend_from_slice(&metadata.previous_opaque_cursor.content_hash);
    bytes.extend_from_slice(&metadata.previous_logical_anchor.height.to_be_bytes());
    bytes.extend_from_slice(
        &metadata
            .previous_logical_anchor
            .cursor
            .sequence
            .to_be_bytes(),
    );
    bytes.extend_from_slice(&metadata.previous_logical_anchor.cursor.content_hash);
    bytes.extend_from_slice(&metadata.previous_logical_anchor.state_binding);
    bytes.extend_from_slice(&metadata.target_height.to_be_bytes());
    bytes.extend_from_slice(&metadata.target_state_binding);
    bytes.extend_from_slice(&metadata.snapshot_manifest_id);
    bytes.extend_from_slice(&metadata.snapshot_body_hash);
    bytes.extend_from_slice(&metadata.proof_manifest_id);
    bytes.extend_from_slice(&metadata.proof_root);
    for height in [
        metadata.proof_genesis_height,
        metadata.execution_start,
        metadata.execution_end,
        metadata.lookahead_height,
        metadata.retained_start,
        metadata.retained_end,
    ] {
        bytes.extend_from_slice(&height.to_be_bytes());
    }
    bytes.extend_from_slice(&target.encoded);
    let checksum: [u8; 32] = Sha256::digest(&bytes).into();
    bytes.extend_from_slice(&checksum);
    Ok(bytes)
}
