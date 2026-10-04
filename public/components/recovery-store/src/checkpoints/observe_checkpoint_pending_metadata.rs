// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::is_valid_checkpoint_pending_completion::is_valid_checkpoint_pending_completion;
use super::{
    CheckpointError, CheckpointManifestPreflight,
    checkpoint_completion_bytes::checkpoint_completion_bytes,
    is_valid_checkpoint_pending_manifest::is_valid_checkpoint_pending_manifest,
    open_checkpoint_file::open_checkpoint_file,
    open_checkpoint_namespace::open_checkpoint_namespace,
    pending_metadata_types::{CheckpointPendingMetadataKind, CheckpointPendingMetadataStatus},
    read_checkpoint_metadata::read_checkpoint_metadata,
    required_checkpoint_metadata_reservation,
    validate_checkpoint_directory::validate_checkpoint_directory,
    validate_checkpoint_file::validate_checkpoint_file,
};
use std::fs::File;

/// Read only, without namespace creation, promotion or repair. Caller holds a real metadata lease.
pub fn observe_checkpoint_pending_metadata(
    base: &File,
    manifest: &CheckpointManifestPreflight<'_>,
    kind: CheckpointPendingMetadataKind,
    reserved_metadata: usize,
) -> Result<CheckpointPendingMetadataStatus, CheckpointError> {
    if reserved_metadata < required_checkpoint_metadata_reservation(&manifest.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let directory = match open_checkpoint_namespace(base, &manifest.summary.id, false) {
        Ok(directory) => directory,
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(CheckpointPendingMetadataStatus::Missing);
        }
        Err(error) => return Err(error),
    };
    validate_checkpoint_directory(&directory, manifest.summary.chunks)?;
    match open_checkpoint_file(&directory, "complete.bin", false) {
        Ok(_) => return Ok(CheckpointPendingMetadataStatus::PublishedCompletionPresent),
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    let (name, maximum) = match kind {
        CheckpointPendingMetadataKind::Manifest => {
            ("manifest.pending", manifest.limits.maximum_manifest_bytes)
        }
        CheckpointPendingMetadataKind::Completion => ("complete.pending", 80),
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
    let completion = checkpoint_completion_bytes(&manifest.summary);
    let expected = match kind {
        CheckpointPendingMetadataKind::Manifest => manifest.bytes,
        CheckpointPendingMetadataKind::Completion => completion.as_slice(),
    };
    if bytes == expected {
        return Ok(CheckpointPendingMetadataStatus::MatchingValid);
    }
    let preserved = match kind {
        CheckpointPendingMetadataKind::Manifest => is_valid_checkpoint_pending_manifest(&bytes),
        CheckpointPendingMetadataKind::Completion => is_valid_checkpoint_pending_completion(&bytes),
    };
    Ok(if preserved {
        CheckpointPendingMetadataStatus::Refused
    } else {
        CheckpointPendingMetadataStatus::InvalidKnown
    })
}
