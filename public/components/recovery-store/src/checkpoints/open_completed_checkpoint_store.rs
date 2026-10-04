// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointLimits, CheckpointResumeObservation, CheckpointTransfer,
    CompletedCheckpointStore, checkpoint_completion_bytes::checkpoint_completion_bytes,
    open_checkpoint_namespace::open_checkpoint_namespace, preflight_checkpoint_manifest,
    read_checkpoint_metadata::read_checkpoint_metadata, required_checkpoint_io_reservation,
    required_checkpoint_metadata_reservation,
    validate_checkpoint_chunks::validate_checkpoint_chunks,
    validate_checkpoint_directory::validate_checkpoint_directory,
};
use eve_state::{StateVersion, encode_state_version};
use std::fs::File;

/// Reopen only an already completed exact namespace. Never creates, republishes or repairs content.
pub fn open_completed_checkpoint_store(
    base: &File,
    expected_manifest_id: &[u8; 32],
    expected_target: &StateVersion,
    limits: &CheckpointLimits,
    reserved_metadata: usize,
    reserved_io: usize,
) -> Result<CompletedCheckpointStore, CheckpointError> {
    if reserved_metadata < required_checkpoint_metadata_reservation(limits)?
        || reserved_io < required_checkpoint_io_reservation(limits)?
    {
        return Err(CheckpointError::ResourceReservation);
    }
    if *expected_manifest_id == [0; 32] {
        return Err(CheckpointError::InvalidManifest);
    }
    let target = encode_state_version(expected_target).map_err(CheckpointError::State)?;
    let directory = open_checkpoint_namespace(base, expected_manifest_id, false)?;
    let bytes =
        read_checkpoint_metadata(&directory, "manifest.bin", limits.maximum_manifest_bytes)?;
    let manifest = preflight_checkpoint_manifest(&bytes, &target, limits)?;
    if manifest.summary.id != *expected_manifest_id {
        return Err(CheckpointError::TargetMismatch);
    }
    let summary = manifest.summary;
    validate_checkpoint_directory(&directory, summary.chunks)?;
    if read_checkpoint_metadata(&directory, "complete.bin", 80)?
        != checkpoint_completion_bytes(&summary)
    {
        return Err(CheckpointError::NamespaceOccupied);
    }
    let transfer = CheckpointTransfer {
        directory,
        manifest: bytes,
        summary,
        limits: *limits,
        initial_resume: CheckpointResumeObservation {
            verified_chunks: summary.chunks,
            missing_chunks: 0,
            corrupt_chunks: 0,
        },
    };
    validate_checkpoint_chunks(&transfer)?;
    Ok(CompletedCheckpointStore { transfer })
}
