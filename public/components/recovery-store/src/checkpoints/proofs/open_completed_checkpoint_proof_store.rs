// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofLimits, CheckpointProofTransfer, CompletedCheckpointProofStore,
    checkpoint_proof_completion_bytes::checkpoint_proof_completion_bytes,
    checkpoint_proof_store_identity::CheckpointProofStoreIdentity,
    preflight_checkpoint_proof_manifest, required_checkpoint_proof_io_reservation,
    required_checkpoint_proof_metadata_reservation,
    validate_checkpoint_proof_directory::validate_checkpoint_proof_directory,
    validate_checkpoint_proof_witnesses::validate_checkpoint_proof_witnesses,
};
use crate::checkpoints::{
    CheckpointError, CheckpointResumeObservation,
    open_checkpoint_namespace::open_checkpoint_namespace,
    read_checkpoint_metadata::read_checkpoint_metadata,
};
use eve_state::{StateVersion, encode_state_version};
use std::fs::File;

/// Reopen exact completed opaque proof bytes. Does not authenticate them or create missing completion.
pub fn open_completed_checkpoint_proof_store(
    base: &File,
    expected: &CheckpointProofStoreIdentity,
    expected_target: &StateVersion,
    limits: &CheckpointProofLimits,
    reserved_metadata: usize,
    reserved_io: usize,
) -> Result<CompletedCheckpointProofStore, CheckpointError> {
    if reserved_metadata < required_checkpoint_proof_metadata_reservation(limits)?
        || reserved_io < required_checkpoint_proof_io_reservation(limits)?
    {
        return Err(CheckpointError::ResourceReservation);
    }
    if expected.manifest_id == [0; 32] || expected.snapshot_manifest_id == [0; 32] {
        return Err(CheckpointError::InvalidManifest);
    }
    let target = encode_state_version(expected_target).map_err(CheckpointError::State)?;
    let directory = open_checkpoint_namespace(base, &expected.manifest_id, false)?;
    let bytes =
        read_checkpoint_metadata(&directory, "manifest.bin", limits.maximum_manifest_bytes)?;
    let manifest = preflight_checkpoint_proof_manifest(
        &bytes,
        expected_target.height,
        &target,
        &expected.snapshot_manifest_id,
        &expected.snapshot_body_sha256,
        limits,
    )?;
    if manifest.summary.id != expected.manifest_id
        || manifest.summary.stream_hash != expected.stream_sha256
    {
        return Err(CheckpointError::TargetMismatch);
    }
    let summary = manifest.summary;
    validate_checkpoint_proof_directory(&directory, summary.files)?;
    if read_checkpoint_metadata(&directory, "complete.bin", 80)?
        != checkpoint_proof_completion_bytes(&summary)
    {
        return Err(CheckpointError::NamespaceOccupied);
    }
    let transfer = CheckpointProofTransfer {
        directory,
        manifest: bytes,
        summary,
        limits: *limits,
        initial_resume: CheckpointResumeObservation {
            verified_chunks: summary.files,
            missing_chunks: 0,
            corrupt_chunks: 0,
        },
    };
    validate_checkpoint_proof_witnesses(&transfer)?;
    Ok(CompletedCheckpointProofStore { transfer })
}
