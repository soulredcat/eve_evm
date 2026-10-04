// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofManifestPreflight, CheckpointProofPendingKind,
    checkpoint_proof_completion_bytes::checkpoint_proof_completion_bytes,
    is_valid_checkpoint_proof_pending::is_valid_checkpoint_proof_pending,
    required_checkpoint_proof_metadata_reservation,
    validate_checkpoint_proof_directory::validate_checkpoint_proof_directory,
};
use crate::checkpoints::{
    CheckpointError, open_checkpoint_file::open_checkpoint_file,
    open_checkpoint_namespace::open_checkpoint_namespace,
    pending_metadata_types::CheckpointPendingMetadataStatus,
    read_checkpoint_metadata::read_checkpoint_metadata,
    validate_checkpoint_file::validate_checkpoint_file,
};
use std::fs::File;

/// Protected bounded metadata observation only; published marker presence is not proof verification.
pub fn observe_checkpoint_proof_pending_metadata(
    base: &File,
    manifest: &CheckpointProofManifestPreflight<'_>,
    kind: CheckpointProofPendingKind,
    reserved_metadata: usize,
) -> Result<CheckpointPendingMetadataStatus, CheckpointError> {
    if reserved_metadata < required_checkpoint_proof_metadata_reservation(&manifest.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let directory = match open_checkpoint_namespace(base, &manifest.summary.id, false) {
        Ok(directory) => directory,
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(CheckpointPendingMetadataStatus::Missing);
        }
        Err(error) => return Err(error),
    };
    validate_checkpoint_proof_directory(&directory, manifest.summary.files)?;
    match open_checkpoint_file(&directory, "complete.bin", false) {
        Ok(_) => return Ok(CheckpointPendingMetadataStatus::PublishedCompletionPresent),
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    let (name, maximum) = match kind {
        CheckpointProofPendingKind::Manifest => {
            ("manifest.pending", manifest.limits.maximum_manifest_bytes)
        }
        CheckpointProofPendingKind::Completion => ("complete.pending", 80),
    };
    let file = match open_checkpoint_file(&directory, name, false) {
        Ok(file) => file,
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(CheckpointPendingMetadataStatus::Missing);
        }
        Err(error) => return Err(error),
    };
    if validate_checkpoint_file(&file)? > maximum as u64 {
        return Ok(CheckpointPendingMetadataStatus::Refused);
    }
    let bytes = read_checkpoint_metadata(&directory, name, maximum)?;
    let completion = checkpoint_proof_completion_bytes(&manifest.summary);
    let expected = match kind {
        CheckpointProofPendingKind::Manifest => manifest.bytes,
        CheckpointProofPendingKind::Completion => completion.as_slice(),
    };
    if bytes == expected {
        return Ok(CheckpointPendingMetadataStatus::MatchingValid);
    }
    Ok(if is_valid_checkpoint_proof_pending(&bytes, kind) {
        CheckpointPendingMetadataStatus::Refused
    } else {
        CheckpointPendingMetadataStatus::InvalidKnown
    })
}
