// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_evm::execute_state_block;
use eve_state::{StateBudget, validate_retained_block};

use super::derive_authenticated_execution_input::derive_authenticated_execution_input;
use crate::{
    authenticate_current_application_version,
    recovery::{
        CompactRecoveryEnvelopeV1, DevelopmentRecoveryState, RecoveryError,
        VerifiedRecoveryTransition, validate_recovery_envelope_bounds,
        verification::{
            validate_recovery_parent::validate_recovery_parent,
            verify_recovery_history::verify_recovery_history,
        },
    },
};

/// Prepare without publication or I/O. The caller must hold a real resource charge;
/// reserved_clone_bytes is only the canonical executor's checked resource parameter.
pub fn prepare_development_recovery(
    parent: &DevelopmentRecoveryState,
    envelope: Arc<CompactRecoveryEnvelopeV1>,
    budget: &StateBudget,
    reserved_clone_bytes: usize,
) -> Result<VerifiedRecoveryTransition, RecoveryError> {
    validate_recovery_envelope_bounds(&envelope)?;
    validate_recovery_parent(parent, &envelope)?;
    validate_retained_block(
        &envelope.expected,
        Some(&envelope.parent),
        &envelope.execution,
        budget,
    )
    .map_err(RecoveryError::State)?;
    let history = verify_recovery_history(parent, &envelope)?;
    let input = derive_authenticated_execution_input(&history.finalized, &parent.policy)?;
    let prepared = execute_state_block(
        &parent.commit,
        &input,
        &envelope.execution.transactions,
        budget,
        reserved_clone_bytes,
    )
    .map_err(RecoveryError::Execution)?;
    if prepared.commit.target != envelope.expected || prepared.commit.block != envelope.execution {
        return Err(RecoveryError::ReplayMismatch);
    }
    let anchor =
        authenticate_current_application_version(&history.finality, &prepared.commit.target)
            .map_err(RecoveryError::Finality)?;
    let state = DevelopmentRecoveryState {
        commit: Arc::new(prepared.commit),
        finality: history.finality,
        policy: Arc::clone(&parent.policy),
        lookahead: Some(Arc::new(envelope.lookahead.clone())),
        lookahead_header: Some(history.lookahead),
        anchor: Some(anchor),
    };
    Ok(VerifiedRecoveryTransition {
        state: Arc::new(state),
        envelope,
    })
}
