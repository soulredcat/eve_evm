// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    ChargedCheckpointArtifacts, CheckpointAppliedError,
    verify_applied_checkpoint_witness::verify_applied_checkpoint_witness,
};
use crate::sync::applied::{resources::reserve_estimated_working, state::applied_state_commit};
use eve_finality_verifier::{
    begin_authenticated_checkpoint, finish_authenticated_checkpoint, imported_state_commit,
    initialize_authenticated_import, required_checkpoint_reservation,
};
use eve_state::{
    BOUNDED_STATE_CODEC_SCRATCH_BYTES, DevelopmentGenesis, StateCommit,
    estimate_genesis_initialization_reservation,
};
use std::sync::Arc;

/// Reopen recoverability requires every retained proof, even the actual parent's older prefix.
pub(super) fn validate_full_checkpoint_stream(
    artifacts: &ChargedCheckpointArtifacts,
    target: &Arc<StateCommit>,
    local_configured_genesis: &DevelopmentGenesis,
) -> Result<(), CheckpointAppliedError> {
    let budget = &artifacts.content.limits.content.logical;
    let _scratch = reserve_estimated_working(
        &artifacts.content.working,
        BOUNDED_STATE_CODEC_SCRATCH_BYTES,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let required = estimate_genesis_initialization_reservation(local_configured_genesis, budget)
        .map_err(CheckpointAppliedError::State)?;
    let _genesis_lease = reserve_estimated_working(&artifacts.content.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    let genesis =
        initialize_authenticated_import(local_configured_genesis, budget).map_err(|error| {
            CheckpointAppliedError::Applied(crate::sync::applied::AppliedError::Import(error))
        })?;
    if imported_state_commit(&genesis).target.identity
        != applied_state_commit(&artifacts.content.parent.generation.state)
            .target
            .identity
    {
        return Err(CheckpointAppliedError::InvalidArtifactBinding);
    }
    let required = required_checkpoint_reservation(
        &genesis,
        target,
        budget,
        artifacts.content.limits.verification,
    )
    .map_err(CheckpointAppliedError::Checkpoint)?;
    let _session_lease = reserve_estimated_working(&artifacts.content.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    let mut session = begin_authenticated_checkpoint(
        &genesis,
        Arc::clone(target),
        budget,
        artifacts.content.limits.verification,
        required,
    )
    .map_err(CheckpointAppliedError::Checkpoint)?;
    let closing = target
        .target
        .height
        .checked_add(1)
        .ok_or(CheckpointAppliedError::InvalidArtifactBinding)?;
    for height in 1..=closing {
        verify_applied_checkpoint_witness(artifacts, &mut session, height, required)?;
    }
    let verified =
        finish_authenticated_checkpoint(session).map_err(CheckpointAppliedError::Checkpoint)?;
    drop(verified);
    Ok(())
}
