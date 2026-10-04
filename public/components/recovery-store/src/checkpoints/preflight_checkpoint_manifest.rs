// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointLimits, CheckpointManifestPreflight,
    types::{HEADER_BYTES, MAGIC, ManifestSummary, REFERENCE_BYTES},
    validate_checkpoint_limits::validate_checkpoint_limits,
};
use sha2::{Digest, Sha256};

/// Borrowed bounded framing only. Expected canonical version bytes constrain equality, not trust.
pub fn preflight_checkpoint_manifest<'a>(
    bytes: &'a [u8],
    expected_version_encoding: &[u8],
    limits: &CheckpointLimits,
) -> Result<CheckpointManifestPreflight<'a>, CheckpointError> {
    validate_checkpoint_limits(limits)?;
    if bytes.len() < HEADER_BYTES
        || bytes.len() > limits.maximum_manifest_bytes
        || &bytes[..24] != MAGIC
        || bytes[24..26] != [0, 0]
    {
        return Err(CheckpointError::InvalidManifest);
    }
    let version_bytes = usize::from(u16::from_be_bytes(bytes[26..28].try_into().unwrap()));
    let body_bytes = usize::try_from(u64::from_be_bytes(bytes[28..36].try_into().unwrap()))
        .map_err(|_| CheckpointError::InvalidManifest)?;
    let chunk_bytes = usize::try_from(u32::from_be_bytes(bytes[36..40].try_into().unwrap()))
        .map_err(|_| CheckpointError::InvalidManifest)?;
    let chunks = usize::try_from(u32::from_be_bytes(bytes[40..44].try_into().unwrap()))
        .map_err(|_| CheckpointError::InvalidManifest)?;
    if version_bytes == 0
        || version_bytes > 4_096
        || body_bytes == 0
        || body_bytes > limits.maximum_body_bytes
        || chunk_bytes == 0
        || chunk_bytes > limits.maximum_chunk_bytes
        || chunks == 0
        || chunks > limits.maximum_chunks
        || body_bytes.div_ceil(chunk_bytes) != chunks
    {
        return Err(CheckpointError::InvalidManifest);
    }
    let expected_length = chunks
        .checked_mul(REFERENCE_BYTES)
        .and_then(|length| length.checked_add(HEADER_BYTES + version_bytes))
        .ok_or(CheckpointError::InvalidManifest)?;
    if bytes.len() != expected_length {
        return Err(CheckpointError::InvalidManifest);
    }
    if &bytes[HEADER_BYTES..HEADER_BYTES + version_bytes] != expected_version_encoding {
        return Err(CheckpointError::TargetMismatch);
    }
    for index in 0..chunks {
        let position = HEADER_BYTES + version_bytes + index * REFERENCE_BYTES;
        let encoded_index = u32::from_be_bytes(bytes[position..position + 4].try_into().unwrap());
        let offset = u64::from_be_bytes(bytes[position + 4..position + 12].try_into().unwrap());
        let length = u32::from_be_bytes(bytes[position + 12..position + 16].try_into().unwrap());
        let actual_offset = index
            .checked_mul(chunk_bytes)
            .ok_or(CheckpointError::InvalidManifest)?;
        if usize::try_from(encoded_index).ok() != Some(index)
            || usize::try_from(offset).ok() != Some(actual_offset)
            || usize::try_from(length).ok() != Some(chunk_bytes.min(body_bytes - actual_offset))
        {
            return Err(CheckpointError::InvalidManifest);
        }
    }
    Ok(CheckpointManifestPreflight {
        bytes,
        limits: *limits,
        summary: ManifestSummary {
            id: Sha256::digest(bytes).into(),
            body_hash: bytes[44..76].try_into().unwrap(),
            body_bytes,
            chunk_bytes,
            chunks,
            version_bytes,
        },
    })
}
