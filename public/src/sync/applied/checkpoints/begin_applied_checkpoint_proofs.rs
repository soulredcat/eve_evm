// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ChargedCheckpointProofTransfer, ChargedCompletedCheckpoint, CheckpointAppliedError};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_storage::checkpoints::{
    checkpoint_target_encoding,
    proofs::{
        begin_checkpoint_proof_transfer, checkpoint_proof_manifest_id,
        checkpoint_proof_manifest_stats, preflight_checkpoint_proof_manifest,
        required_checkpoint_proof_metadata_reservation,
    },
};
use std::fs::File;

/// Caller charges borrowed ingress independently; copied metadata uses the captured owner's pool.
pub fn begin_applied_checkpoint_proofs(
    content: ChargedCompletedCheckpoint,
    base: &File,
    manifest_bytes: &[u8],
) -> Result<ChargedCheckpointProofTransfer, CheckpointAppliedError> {
    let required = required_checkpoint_proof_metadata_reservation(&content.limits.proofs)
        .map_err(CheckpointAppliedError::Storage)?;
    let metadata = reserve_estimated_working(&content.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    let staging = super::super::resources::storage_admission::reserve_snapshot_staging(
        &content.storage,
        required,
    )
    .map_err(super::super::resources::storage_admission::map_storage_admission_error)
    .map_err(CheckpointAppliedError::Applied)?;
    let _read = super::super::resources::storage_admission::reserve_storage_read(&content.storage)
        .map_err(super::super::resources::storage_admission::map_storage_admission_error)
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
    let proof_id = checkpoint_proof_manifest_id(&manifest);
    let proofs = checkpoint_proof_manifest_stats(&manifest);
    let transfer = begin_checkpoint_proof_transfer(base, &manifest, required)
        .map_err(CheckpointAppliedError::Storage)?;
    Ok(ChargedCheckpointProofTransfer {
        content,
        transfer,
        proof_id,
        proofs,
        _metadata: metadata,
        _staging: staging,
    })
}
