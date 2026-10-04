// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofLimits, CheckpointProofManifestPreflight,
    types::{HEADER_BYTES, MAGIC, ProofSummary, REFERENCE_BYTES},
    validate_checkpoint_proof_limits::validate_checkpoint_proof_limits,
};
use crate::checkpoints::CheckpointError;
use sha2::{Digest, Sha256};

/// Allocation-free framing/equality only; expected height/encoding must come from one caller version.
pub fn preflight_checkpoint_proof_manifest<'a>(
    bytes: &'a [u8],
    expected_height: u64,
    expected_target: &[u8],
    expected_snapshot_id: &[u8; 32],
    expected_body_sha256: &[u8; 32],
    limits: &CheckpointProofLimits,
) -> Result<CheckpointProofManifestPreflight<'a>, CheckpointError> {
    validate_checkpoint_proof_limits(limits)?;
    if bytes.len() < HEADER_BYTES
        || bytes.len() > limits.maximum_manifest_bytes
        || &bytes[..24] != MAGIC
        || bytes[24..26] != [0, 0]
    {
        return Err(CheckpointError::InvalidManifest);
    }
    let version_bytes = usize::from(u16::from_be_bytes(bytes[26..28].try_into().unwrap()));
    let height = u64::from_be_bytes(bytes[28..36].try_into().unwrap());
    let files = usize::try_from(u32::from_be_bytes(bytes[36..40].try_into().unwrap()))
        .map_err(|_| CheckpointError::InvalidManifest)?;
    let total_bytes = usize::try_from(u64::from_be_bytes(bytes[40..48].try_into().unwrap()))
        .map_err(|_| CheckpointError::InvalidManifest)?;
    if version_bytes == 0
        || version_bytes > 4_096
        || height >= 10_001
        || files == 0
        || files > limits.maximum_files
        || files != height as usize + 1
        || total_bytes == 0
        || total_bytes > limits.maximum_total_bytes
    {
        return Err(CheckpointError::InvalidManifest);
    }
    let length = files
        .checked_mul(REFERENCE_BYTES)
        .and_then(|length| length.checked_add(HEADER_BYTES + version_bytes))
        .ok_or(CheckpointError::InvalidManifest)?;
    if bytes.len() != length {
        return Err(CheckpointError::InvalidManifest);
    }
    let disk = total_bytes
        .checked_add(
            length
                .checked_mul(2)
                .ok_or(CheckpointError::ResourceReservation)?,
        )
        .and_then(|total| total.checked_add(160))
        .ok_or(CheckpointError::ResourceReservation)?;
    if disk > limits.maximum_disk_bytes {
        return Err(CheckpointError::ResourceReservation);
    }
    if height != expected_height
        || &bytes[48..80] != expected_snapshot_id
        || &bytes[80..112] != expected_body_sha256
        || &bytes[HEADER_BYTES..HEADER_BYTES + version_bytes] != expected_target
    {
        return Err(CheckpointError::TargetMismatch);
    }
    let mut total = 0_usize;
    for index in 0..files {
        let position = HEADER_BYTES + version_bytes + index * REFERENCE_BYTES;
        let reference = &bytes[position..position + REFERENCE_BYTES];
        let encoded_height = u64::from_be_bytes(reference[..8].try_into().unwrap());
        let length = usize::try_from(u64::from_be_bytes(reference[16..24].try_into().unwrap()))
            .map_err(|_| CheckpointError::InvalidManifest)?;
        if encoded_height != index as u64 + 1
            || reference[8] != u8::from(index + 1 == files)
            || reference[9..16] != [0; 7]
            || length == 0
            || length > limits.maximum_witness_bytes
        {
            return Err(CheckpointError::InvalidManifest);
        }
        total = total
            .checked_add(length)
            .ok_or(CheckpointError::ResourceReservation)?;
    }
    if total != total_bytes {
        return Err(CheckpointError::InvalidManifest);
    }
    let mut id = Sha256::new();
    id.update(b"EVE_CHECKPOINT_PROOF_MANIFEST_V1");
    id.update(bytes);
    Ok(CheckpointProofManifestPreflight {
        bytes,
        limits: *limits,
        summary: ProofSummary {
            id: id.finalize().into(),
            stream_hash: bytes[112..144].try_into().unwrap(),
            height,
            files,
            total_bytes,
            version_bytes,
        },
    })
}
