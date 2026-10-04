// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{
    AppliedConfig, AppliedError, AppliedMode,
    resources::{
        EstimatedWorkingPool, estimate_replay_charge, estimated_clone_ceiling,
        reserve_estimated_working, split_estimated_working,
    },
    state::AppliedState,
    types::ChargedAppliedState,
};
use eve_finality_verifier::{initialize_authenticated_import, initialize_development_recovery};
use eve_state::{
    BOUNDED_STATE_CODEC_SCRATCH_BYTES, DevelopmentGenesis,
    estimate_genesis_initialization_reservation,
};
use std::sync::Arc;

pub(in crate::sync::applied) fn initialize_charged_generation(
    config: &AppliedConfig,
    genesis: &DevelopmentGenesis,
    mode: AppliedMode,
    working: &Arc<EstimatedWorkingPool>,
) -> Result<Arc<ChargedAppliedState>, AppliedError> {
    match mode {
        AppliedMode::EmptyReplay => {
            let charge = estimate_replay_charge(
                &config.state_budget,
                config.maximum_recovery_payload_bytes,
                estimated_clone_ceiling(&config.state_budget)?,
            )?;
            let lease = reserve_estimated_working(working, charge.total)?;
            let state = initialize_development_recovery(genesis, &config.state_budget)
                .map_err(AppliedError::Recovery)?;
            let (retained, transient) = split_estimated_working(lease, charge.retained)?;
            let generation = Arc::new(ChargedAppliedState {
                state: AppliedState::EmptyReplay(Arc::new(state)),
                _lease: retained,
            });
            drop(transient);
            Ok(generation)
        }
        AppliedMode::AuthenticatedImport => {
            let scratch = reserve_estimated_working(working, BOUNDED_STATE_CODEC_SCRATCH_BYTES)?;
            let required =
                estimate_genesis_initialization_reservation(genesis, &config.state_budget)
                    .map_err(|_| AppliedError::EstimatedCapacity)?;
            let lease = reserve_estimated_working(working, required)?;
            let state = initialize_authenticated_import(genesis, &config.state_budget)
                .map_err(AppliedError::Import)?;
            let generation = Arc::new(ChargedAppliedState {
                state: AppliedState::AuthenticatedImport(Arc::new(state)),
                _lease: lease,
            });
            drop(scratch);
            Ok(generation)
        }
    }
}
