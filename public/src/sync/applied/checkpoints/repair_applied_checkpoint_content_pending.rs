// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    AppliedCheckpointLimits, CheckpointAppliedError,
    pending_metadata_types::CheckpointMetadataRepairOutcome,
    repair_applied_checkpoint_content_entry::repair_applied_checkpoint_content_entry,
};
use crate::sync::applied::{
    AppliedOwner, capture_applied_state, resources::reserve_estimated_working, state::AppliedState,
    types::AppliedBackend,
};
use eve_storage::checkpoints::{
    CheckpointPendingMetadataKind, CheckpointPendingMetadataStatus,
    observe_checkpoint_pending_metadata, preflight_checkpoint_manifest,
    required_checkpoint_metadata_reservation,
};
use std::fs::File;

/// Explicit local staging repair. The actual owner lease surrounds both read-only probes and repair.
pub fn repair_applied_checkpoint_content_pending(
    owner: &AppliedOwner,
    base: &File,
    manifest_bytes: &[u8],
    expected_target_encoding: &[u8],
    limits: AppliedCheckpointLimits,
) -> Result<CheckpointMetadataRepairOutcome, CheckpointAppliedError> {
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
    let _storage = crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
        &owner.reader.storage,
        required,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let _metadata = reserve_estimated_working(&owner.reader.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    let manifest =
        preflight_checkpoint_manifest(manifest_bytes, expected_target_encoding, &limits.content)
            .map_err(CheckpointAppliedError::Storage)?;
    let manifest_status = observe_checkpoint_pending_metadata(
        base,
        &manifest,
        CheckpointPendingMetadataKind::Manifest,
        required,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    let completion_status = observe_checkpoint_pending_metadata(
        base,
        &manifest,
        CheckpointPendingMetadataKind::Completion,
        required,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    if manifest_status == CheckpointPendingMetadataStatus::Refused
        || completion_status == CheckpointPendingMetadataStatus::Refused
    {
        return Err(CheckpointAppliedError::PendingMetadataRefused);
    }
    let manifest_outcome = repair_applied_checkpoint_content_entry(
        base,
        &manifest,
        CheckpointPendingMetadataKind::Manifest,
        manifest_status,
        required,
    )?;
    let completion = repair_applied_checkpoint_content_entry(
        base,
        &manifest,
        CheckpointPendingMetadataKind::Completion,
        completion_status,
        required,
    )?;
    Ok(CheckpointMetadataRepairOutcome {
        manifest: manifest_outcome,
        completion,
    })
}
