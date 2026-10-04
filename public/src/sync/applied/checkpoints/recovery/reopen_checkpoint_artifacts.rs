// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    open_checkpoint_recovery_root::open_checkpoint_recovery_root, types::DiscoveredCheckpointBase,
};
use crate::sync::applied::{
    AppliedPublication,
    checkpoints::{
        ChargedCheckpointArtifacts, ChargedCompletedCheckpoint, CheckpointAppliedError,
        CheckpointRecoveryConfig,
    },
    resources::{EstimatedWorkingPool, reserve_estimated_working},
};
use eve_storage::{
    checkpoints::{
        checkpoint_manifest_id, checkpoint_manifest_stats, checkpoint_store_manifest_bytes,
        open_completed_checkpoint_store, preflight_checkpoint_manifest,
        proofs::{
            CheckpointProofStoreIdentity, checkpoint_proof_manifest_id,
            checkpoint_proof_manifest_stats, checkpoint_proof_store_manifest_bytes,
            open_completed_checkpoint_proof_store, preflight_checkpoint_proof_manifest,
            required_checkpoint_proof_io_reservation,
            required_checkpoint_proof_metadata_reservation,
        },
        required_checkpoint_io_reservation, required_checkpoint_metadata_reservation,
    },
    records::segmented::checkpoints::{
        checkpoint_base_membership_view, prepared_checkpoint_base_target_bytes,
    },
};
use std::sync::Arc;

/// Open only exact completed existing artifact namespaces; missing completion is never recreated.
pub(super) fn reopen_checkpoint_artifacts(
    base: &DiscoveredCheckpointBase,
    config: &CheckpointRecoveryConfig,
    parent: Arc<AppliedPublication>,
    working: &Arc<EstimatedWorkingPool>,
    storage: &Arc<crate::sync::applied::resources::storage_admission::StorageAdmissionPool>,
) -> Result<ChargedCheckpointArtifacts, CheckpointAppliedError> {
    let metadata = checkpoint_base_membership_view(&base.membership).metadata;
    let target = prepared_checkpoint_base_target_bytes(&base.target);
    let content_required = required_checkpoint_metadata_reservation(&config.limits.content)
        .map_err(CheckpointAppliedError::Storage)?;
    let content_lease = reserve_estimated_working(working, content_required)
        .map_err(CheckpointAppliedError::Applied)?;
    let content_staging =
        crate::sync::applied::resources::storage_admission::reserve_snapshot_staging(
            storage,
            content_required,
        )
        .map_err(crate::sync::applied::resources::storage_admission::map_storage_admission_error)
        .map_err(CheckpointAppliedError::Applied)?;
    let content_io = required_checkpoint_io_reservation(&config.limits.content)
        .map_err(CheckpointAppliedError::Storage)?;
    let _content_io =
        reserve_estimated_working(working, content_io).map_err(CheckpointAppliedError::Applied)?;
    let content_job =
        crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
            storage, content_io,
        )
        .map_err(CheckpointAppliedError::Applied)?;
    let content_root = open_checkpoint_recovery_root(&config.content_root)?;
    let content_store = open_completed_checkpoint_store(
        &content_root,
        &metadata.snapshot_manifest_id,
        &base.version,
        &config.limits.content,
        content_required,
        content_io,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    let manifest = preflight_checkpoint_manifest(
        checkpoint_store_manifest_bytes(&content_store),
        target,
        &config.limits.content,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    let content_id = checkpoint_manifest_id(&manifest);
    let content_stats = checkpoint_manifest_stats(&manifest);
    if content_id != metadata.snapshot_manifest_id
        || content_stats.body_sha256 != metadata.snapshot_body_hash
    {
        return Err(CheckpointAppliedError::InvalidArtifactBinding);
    }
    let content = ChargedCompletedCheckpoint {
        storage: Arc::clone(storage),
        content_store,
        parent,
        working: Arc::clone(working),
        limits: config.limits,
        content_id,
        content: content_stats,
        _metadata: content_lease,
        _staging: content_staging,
    };
    drop(content_job);
    let proof_required = required_checkpoint_proof_metadata_reservation(&config.limits.proofs)
        .map_err(CheckpointAppliedError::Storage)?;
    let proof_lease = reserve_estimated_working(working, proof_required)
        .map_err(CheckpointAppliedError::Applied)?;
    let proof_staging =
        crate::sync::applied::resources::storage_admission::reserve_snapshot_staging(
            storage,
            proof_required,
        )
        .map_err(crate::sync::applied::resources::storage_admission::map_storage_admission_error)
        .map_err(CheckpointAppliedError::Applied)?;
    let proof_io = required_checkpoint_proof_io_reservation(&config.limits.proofs)
        .map_err(CheckpointAppliedError::Storage)?;
    let _proof_io =
        reserve_estimated_working(working, proof_io).map_err(CheckpointAppliedError::Applied)?;
    let _proof_job =
        crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
            storage, proof_io,
        )
        .map_err(CheckpointAppliedError::Applied)?;
    let proof_root = open_checkpoint_recovery_root(&config.proof_root)?;
    let proof_store = open_completed_checkpoint_proof_store(
        &proof_root,
        &CheckpointProofStoreIdentity {
            manifest_id: metadata.proof_manifest_id,
            snapshot_manifest_id: metadata.snapshot_manifest_id,
            snapshot_body_sha256: metadata.snapshot_body_hash,
            stream_sha256: metadata.proof_root,
        },
        &base.version,
        &config.limits.proofs,
        proof_required,
        proof_io,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    let sealed = preflight_checkpoint_proof_manifest(
        checkpoint_proof_store_manifest_bytes(&proof_store),
        base.version.height,
        target,
        &metadata.snapshot_manifest_id,
        &metadata.snapshot_body_hash,
        &config.limits.proofs,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    let proof_id = checkpoint_proof_manifest_id(&sealed);
    let proofs = checkpoint_proof_manifest_stats(&sealed);
    if proof_id != metadata.proof_manifest_id || proofs.stream_sha256 != metadata.proof_root {
        return Err(CheckpointAppliedError::InvalidArtifactBinding);
    }
    Ok(ChargedCheckpointArtifacts {
        content,
        proof_store,
        proof_id,
        proofs,
        _metadata: proof_lease,
        _staging: proof_staging,
    })
}
