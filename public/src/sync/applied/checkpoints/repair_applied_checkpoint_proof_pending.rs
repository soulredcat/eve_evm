// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    ChargedCompletedCheckpoint, CheckpointAppliedError,
    pending_metadata_types::CheckpointMetadataRepairOutcome,
    repair_applied_checkpoint_proof_entry::repair_applied_checkpoint_proof_entry,
};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_storage::checkpoints::{
    CheckpointPendingMetadataStatus, checkpoint_target_encoding,
    proofs::{
        CheckpointProofPendingKind, observe_checkpoint_proof_pending_metadata,
        preflight_checkpoint_proof_manifest, required_checkpoint_proof_metadata_reservation,
    },
};
use std::fs::File;

/// Borrowed ingress remains caller-charged. The captured owner pool covers every local metadata read.
pub fn repair_applied_checkpoint_proof_pending(
    content: &ChargedCompletedCheckpoint,
    base: &File,
    manifest_bytes: &[u8],
) -> Result<CheckpointMetadataRepairOutcome, CheckpointAppliedError> {
    let required = required_checkpoint_proof_metadata_reservation(&content.limits.proofs)
        .map_err(CheckpointAppliedError::Storage)?;
    let _storage = crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
        &content.storage,
        required,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let _metadata = reserve_estimated_working(&content.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    let target_bytes = checkpoint_target_encoding(&content.content_store);
    let target =
        eve_state::decode_state_version(target_bytes).map_err(CheckpointAppliedError::State)?;
    let manifest = preflight_checkpoint_proof_manifest(
        manifest_bytes,
        target.height,
        target_bytes,
        &content.content_id,
        &content.content.body_sha256,
        &content.limits.proofs,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    let manifest_status = observe_checkpoint_proof_pending_metadata(
        base,
        &manifest,
        CheckpointProofPendingKind::Manifest,
        required,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    let completion_status = observe_checkpoint_proof_pending_metadata(
        base,
        &manifest,
        CheckpointProofPendingKind::Completion,
        required,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    if manifest_status == CheckpointPendingMetadataStatus::Refused
        || completion_status == CheckpointPendingMetadataStatus::Refused
    {
        return Err(CheckpointAppliedError::PendingMetadataRefused);
    }
    let manifest_outcome = repair_applied_checkpoint_proof_entry(
        base,
        &manifest,
        CheckpointProofPendingKind::Manifest,
        manifest_status,
        required,
    )?;
    let completion = repair_applied_checkpoint_proof_entry(
        base,
        &manifest,
        CheckpointProofPendingKind::Completion,
        completion_status,
        required,
    )?;
    Ok(CheckpointMetadataRepairOutcome {
        manifest: manifest_outcome,
        completion,
    })
}
