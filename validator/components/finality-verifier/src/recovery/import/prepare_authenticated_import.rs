// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_state::{
    StateBudget, apply_state_journal_reserved, build_state_commit,
    estimate_journal_candidate_reservation,
};

use super::{
    AuthenticatedImportInput, ImportError, ImportedState, ImportedTransition,
    validate_import_execution_context::validate_import_execution_context,
    validate_import_execution_history::validate_import_execution_history,
    validate_import_input_bounds::validate_import_input_bounds,
    validate_import_parent::validate_import_parent,
};
use crate::{
    authenticate_current_application_version,
    recovery::{
        RecoveryError,
        verification::verify_authenticated_recovery_history::{
            RecoveryHistoryInput, RecoveryHistoryParent, verify_authenticated_recovery_history,
        },
    },
};

/// Authenticate and stage a delta without REVM, publication or I/O. The caller
/// must hold a real candidate/resource lease through every retained allocation.
/// Logical conservative reservations do not prove allocator/RSS containment.
pub fn prepare_authenticated_import(
    parent: &ImportedState,
    input: Arc<AuthenticatedImportInput>,
    budget: &StateBudget,
    reserved_candidate_bytes: usize,
) -> Result<ImportedTransition, ImportError> {
    validate_import_input_bounds(&input, budget)?;
    validate_import_parent(parent, &input)?;
    validate_import_execution_history(&input)?;
    let required =
        estimate_journal_candidate_reservation(&parent.commit.state, &input.journal, budget)
            .map_err(ImportError::State)?;
    if reserved_candidate_bytes < required {
        return Err(ImportError::CandidateReservation {
            required,
            reserved: reserved_candidate_bytes,
        });
    }
    let history = verify_authenticated_recovery_history(
        &RecoveryHistoryParent {
            finality: &parent.finality,
            height: parent.commit.target.height,
            lookahead: parent.lookahead.as_deref(),
            lookahead_header: parent.lookahead_header.as_ref(),
            policy: &parent.policy,
        },
        &RecoveryHistoryInput {
            finalized: &input.finalized,
            transactions: &input.execution.transactions,
            lookahead: &input.lookahead,
        },
    )
    .map_err(ImportError::Recovery)?;
    validate_import_execution_context(&parent.commit, &input, &history.finalized, &parent.policy)?;
    let state = apply_state_journal_reserved(
        &parent.commit.state,
        &parent.commit.target,
        &input.journal,
        budget,
        reserved_candidate_bytes,
    )
    .map_err(ImportError::State)?;
    // Imported account/system changes already include fees. Never apply fees again.
    let commit = build_state_commit(
        Some(parent.commit.target.clone()),
        state,
        input.execution.clone(),
        budget,
    )
    .map_err(ImportError::State)?;
    let anchor = authenticate_current_application_version(&history.finality, &commit.target)
        .map_err(|error| ImportError::Recovery(RecoveryError::Finality(error)))?;
    let state = ImportedState {
        commit: Arc::new(commit),
        finality: history.finality,
        policy: Arc::clone(&parent.policy),
        lookahead: Some(Arc::new(input.lookahead.clone())),
        lookahead_header: Some(history.lookahead),
        anchor: Some(anchor),
    };
    Ok(ImportedTransition {
        state: Arc::new(state),
        input,
    })
}
