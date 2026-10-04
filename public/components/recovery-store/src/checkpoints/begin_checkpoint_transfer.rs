// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointManifestPreflight, CheckpointTransfer,
    open_checkpoint_namespace::open_checkpoint_namespace,
    publish_checkpoint_metadata::publish_checkpoint_metadata,
    required_checkpoint_metadata_reservation, scan_checkpoint_resume::scan_checkpoint_resume,
    validate_checkpoint_directory::validate_checkpoint_directory,
    validate_checkpoint_metadata::validate_checkpoint_metadata,
};
use std::fs::File;

/// Metadata lease must remain held through transfer/completed-store lifetime, including failed tails.
pub fn begin_checkpoint_transfer(
    base: &File,
    manifest: &CheckpointManifestPreflight<'_>,
    reserved_metadata: usize,
) -> Result<CheckpointTransfer, CheckpointError> {
    if reserved_metadata < required_checkpoint_metadata_reservation(&manifest.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    let directory = open_checkpoint_namespace(base, &manifest.summary.id, true)?;
    validate_checkpoint_directory(&directory, manifest.summary.chunks)?;
    publish_checkpoint_metadata(
        &directory,
        "manifest.bin",
        "manifest.pending",
        manifest.bytes,
    )?;
    let mut transfer = CheckpointTransfer {
        directory,
        manifest: manifest.bytes.to_vec(),
        summary: manifest.summary,
        limits: manifest.limits,
        initial_resume: super::CheckpointResumeObservation::default(),
    };
    validate_checkpoint_metadata(&transfer)?;
    transfer.initial_resume = scan_checkpoint_resume(&transfer)?;
    Ok(transfer)
}
