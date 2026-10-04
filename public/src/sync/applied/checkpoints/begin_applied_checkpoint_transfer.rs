// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AppliedCheckpointLimits, ChargedCheckpointTransfer, CheckpointAppliedError};
use crate::sync::applied::{
    AppliedOwner, capture_applied_state, resources::reserve_estimated_working, state::AppliedState,
    types::AppliedBackend,
};
use eve_storage::checkpoints::{
    begin_checkpoint_transfer, checkpoint_manifest_id, checkpoint_manifest_stats,
    preflight_checkpoint_manifest, required_checkpoint_metadata_reservation,
};
use std::{fs::File, sync::Arc};

/// Caller separately charges borrowed ingress buffers. This reserves our owned copies first.
pub fn begin_applied_checkpoint_transfer(
    owner: &AppliedOwner,
    base: &File,
    manifest_bytes: &[u8],
    expected_target_encoding: &[u8],
    limits: AppliedCheckpointLimits,
) -> Result<ChargedCheckpointTransfer, CheckpointAppliedError> {
    if !matches!(owner.backend, AppliedBackend::Segmented { .. })
        || limits.content.logical != owner.config.state_budget
        || limits.proofs.maximum_witness_bytes > limits.verification.maximum_witness_bytes
    {
        return Err(CheckpointAppliedError::WrongMode);
    }
    if owner.storage_failed {
        return Err(CheckpointAppliedError::Applied(
            crate::sync::applied::AppliedError::StorageFailed,
        ));
    }
    let parent = capture_applied_state(&owner.reader).map_err(CheckpointAppliedError::Applied)?;
    if !matches!(
        parent.generation.state,
        AppliedState::AuthenticatedImport(_)
    ) {
        return Err(CheckpointAppliedError::WrongMode);
    }
    let required = required_checkpoint_metadata_reservation(&limits.content)
        .map_err(CheckpointAppliedError::Storage)?;
    let metadata = reserve_estimated_working(&owner.reader.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    let staging = super::super::resources::storage_admission::reserve_snapshot_staging(
        &owner.reader.storage,
        required,
    )
    .map_err(super::super::resources::storage_admission::map_storage_admission_error)
    .map_err(CheckpointAppliedError::Applied)?;
    let _read =
        super::super::resources::storage_admission::reserve_storage_read(&owner.reader.storage)
            .map_err(super::super::resources::storage_admission::map_storage_admission_error)
            .map_err(CheckpointAppliedError::Applied)?;
    let manifest =
        preflight_checkpoint_manifest(manifest_bytes, expected_target_encoding, &limits.content)
            .map_err(CheckpointAppliedError::Storage)?;
    let content_id = checkpoint_manifest_id(&manifest);
    let content = checkpoint_manifest_stats(&manifest);
    let transfer = begin_checkpoint_transfer(base, &manifest, required)
        .map_err(CheckpointAppliedError::Storage)?;
    Ok(ChargedCheckpointTransfer {
        storage: Arc::clone(&owner.reader.storage),
        transfer,
        parent,
        working: Arc::clone(&owner.reader.working),
        limits,
        content_id,
        content,
        _metadata: metadata,
        _staging: staging,
    })
}
