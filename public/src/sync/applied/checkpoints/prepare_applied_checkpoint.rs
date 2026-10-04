// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    ChargedCheckpointArtifacts, CheckpointAppliedError, PreparedAppliedCheckpoint,
    validate_full_checkpoint_stream::validate_full_checkpoint_stream,
    verify_applied_checkpoint_witness::verify_applied_checkpoint_witness,
};
use crate::sync::applied::{
    resources::{merge_estimated_working, reserve_estimated_working},
    segmented::bind_local_state_version,
    state::AppliedState,
    types::ChargedAppliedState,
};
use eve_finality_verifier::{
    begin_authenticated_checkpoint, finish_authenticated_checkpoint, imported_state_commit,
    into_imported_checkpoint_state, required_checkpoint_reservation,
};
use eve_state::{
    DevelopmentGenesis, decode_preflight_state_commit, required_state_commit_decode_reservation,
};
use eve_storage::checkpoints::{
    preflight_checkpoint_body, read_checkpoint_body, required_checkpoint_body_reservation,
};
use std::sync::Arc;

/// Authenticate K+1..H+1 from the captured private K capability; checksums never supply trust.
pub fn prepare_applied_checkpoint(
    artifacts: ChargedCheckpointArtifacts,
    local_configured_genesis: &DevelopmentGenesis,
) -> Result<PreparedAppliedCheckpoint, CheckpointAppliedError> {
    let AppliedState::AuthenticatedImport(parent) = &artifacts.content.parent.generation.state
    else {
        return Err(CheckpointAppliedError::WrongMode);
    };
    let (target, decode_lease) = {
        let body_required = required_checkpoint_body_reservation(&artifacts.content.content_store)
            .map_err(CheckpointAppliedError::Storage)?;
        let _body_lease = reserve_estimated_working(&artifacts.content.working, body_required)
            .map_err(CheckpointAppliedError::Applied)?;
        let _storage =
            crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
                &artifacts.content.storage,
                body_required,
            )
            .map_err(CheckpointAppliedError::Applied)?;
        let body = read_checkpoint_body(&artifacts.content.content_store, body_required)
            .map_err(CheckpointAppliedError::Storage)?;
        let preflight =
            preflight_checkpoint_body(&body).map_err(CheckpointAppliedError::Storage)?;
        let raw_copies = eve_state::state_commit_preflight_bytes(&preflight)
            .len()
            .checked_mul(6)
            .ok_or(CheckpointAppliedError::Applied(
                crate::sync::applied::AppliedError::ArithmeticOverflow,
            ))?;
        let _codec_staging =
            crate::sync::applied::resources::storage_admission::reserve_snapshot_staging(
                &artifacts.content.storage,
                raw_copies,
            )
            .map_err(
                crate::sync::applied::resources::storage_admission::map_storage_admission_error,
            )
            .map_err(CheckpointAppliedError::Applied)?;
        let decode_required = required_state_commit_decode_reservation(&preflight)
            .map_err(CheckpointAppliedError::State)?;
        let decode_lease = reserve_estimated_working(&artifacts.content.working, decode_required)
            .map_err(CheckpointAppliedError::Applied)?;
        let target = Arc::new(
            decode_preflight_state_commit(&preflight).map_err(CheckpointAppliedError::State)?,
        );
        (target, decode_lease)
    };
    if target.target.height != artifacts.proofs.height {
        return Err(CheckpointAppliedError::InvalidArtifactBinding);
    }
    validate_full_checkpoint_stream(&artifacts, &target, local_configured_genesis)?;
    let required = required_checkpoint_reservation(
        parent,
        &target,
        &artifacts.content.limits.content.logical,
        artifacts.content.limits.verification,
    )
    .map_err(CheckpointAppliedError::Checkpoint)?;
    let session_lease = reserve_estimated_working(&artifacts.content.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    let mut session = begin_authenticated_checkpoint(
        parent,
        Arc::clone(&target),
        &artifacts.content.limits.content.logical,
        artifacts.content.limits.verification,
        required,
    )
    .map_err(CheckpointAppliedError::Checkpoint)?;
    let first = imported_state_commit(parent)
        .target
        .height
        .checked_add(1)
        .ok_or(CheckpointAppliedError::InvalidArtifactBinding)?;
    let closing = target
        .target
        .height
        .checked_add(1)
        .ok_or(CheckpointAppliedError::InvalidArtifactBinding)?;
    for height in first..=closing {
        verify_applied_checkpoint_witness(&artifacts, &mut session, height, required)?;
    }
    let authenticated =
        finish_authenticated_checkpoint(session).map_err(CheckpointAppliedError::Checkpoint)?;
    let target_binding =
        bind_local_state_version(&target.target).map_err(CheckpointAppliedError::Applied)?;
    let retained = merge_estimated_working(decode_lease, session_lease)
        .map_err(CheckpointAppliedError::Applied)?;
    let generation = Arc::new(ChargedAppliedState {
        state: AppliedState::AuthenticatedImport(into_imported_checkpoint_state(authenticated)),
        _lease: retained,
    });
    Ok(PreparedAppliedCheckpoint {
        artifacts,
        generation,
        target_binding,
    })
}
