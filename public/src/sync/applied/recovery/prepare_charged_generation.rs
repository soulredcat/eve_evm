// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::prepare_empty_recovery;
use crate::sync::applied::{
    AppliedConfig, AppliedError,
    resources::{
        EstimatedWorkingPool, estimate_replay_charge, reserve_estimated_working,
        split_estimated_working,
    },
    state::{AppliedState, applied_state_commit},
    types::ChargedAppliedState,
};
use eve_evm::estimate_clone_reservation;
use eve_finality_verifier::into_recovery_state;

use std::sync::Arc;

/// Prepare one private charged generation from the exact immutable admission/recovery bytes.
pub(in crate::sync::applied) fn prepare_charged_generation(
    parent: &ChargedAppliedState,
    bytes: &[u8],
    config: &AppliedConfig,
    working: &Arc<EstimatedWorkingPool>,
) -> Result<Arc<ChargedAppliedState>, AppliedError> {
    match &parent.state {
        AppliedState::EmptyReplay(recovery) => {
            let oracle = estimate_clone_reservation(&applied_state_commit(&parent.state).state)
                .map_err(|_| AppliedError::EstimatedCapacity)?;
            let charge = estimate_replay_charge(
                &config.state_budget,
                config.maximum_recovery_payload_bytes,
                oracle,
            )?;
            let lease = reserve_estimated_working(working, charge.total)?;
            let transition = prepare_empty_recovery(recovery, bytes, &config.state_budget, oracle)?;
            let state = AppliedState::EmptyReplay(into_recovery_state(transition));
            let (retained, transient) = split_estimated_working(lease, charge.retained)?;
            let generation = Arc::new(ChargedAppliedState {
                state,
                _lease: retained,
            });
            drop(transient);
            Ok(generation)
        }
        AppliedState::AuthenticatedImport(imported) => {
            super::prepare_charged_import(imported, bytes, &config.state_budget, working, false)
        }
    }
}
