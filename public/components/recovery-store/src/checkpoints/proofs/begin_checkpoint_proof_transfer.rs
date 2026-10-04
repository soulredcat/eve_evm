// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofManifestPreflight, CheckpointProofTransfer,
    required_checkpoint_proof_metadata_reservation,
    scan_checkpoint_proof_resume::scan_checkpoint_proof_resume,
    validate_checkpoint_proof_directory::validate_checkpoint_proof_directory,
    validate_checkpoint_proof_metadata::validate_checkpoint_proof_metadata,
};
use crate::checkpoints::{
    CheckpointError, CheckpointResumeObservation,
    open_checkpoint_namespace::open_checkpoint_namespace,
    publish_checkpoint_metadata::publish_checkpoint_metadata,
};
use std::fs::File;

pub fn begin_checkpoint_proof_transfer(
    base: &File,
    manifest: &CheckpointProofManifestPreflight<'_>,
    reserved_metadata: usize,
) -> Result<CheckpointProofTransfer, CheckpointError> {
    if reserved_metadata < required_checkpoint_proof_metadata_reservation(&manifest.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let directory = open_checkpoint_namespace(base, &manifest.summary.id, true)?;
    validate_checkpoint_proof_directory(&directory, manifest.summary.files)?;
    publish_checkpoint_metadata(
        &directory,
        "manifest.bin",
        "manifest.pending",
        manifest.bytes,
    )?;
    let mut transfer = CheckpointProofTransfer {
        directory,
        manifest: manifest.bytes.to_vec(),
        summary: manifest.summary,
        limits: manifest.limits,
        initial_resume: CheckpointResumeObservation::default(),
    };
    validate_checkpoint_proof_metadata(&transfer)?;
    transfer.initial_resume = scan_checkpoint_proof_resume(&transfer)?;
    Ok(transfer)
}
