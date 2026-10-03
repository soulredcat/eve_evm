// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedError, types::ChargedRecoveryState};
use eve_finality_verifier::{
    VerifiedRecoveryTransition, decode_compact_recovery_envelope, prepare_development_recovery,
    validate_empty_recovery_envelope_bytes,
};
use eve_state::StateBudget;
use std::sync::Arc;

/// Decode and replay only after the caller reserves its actual working and input leases.
pub(in crate::sync::applied) fn prepare_empty_recovery(
    parent: &ChargedRecoveryState,
    bytes: &[u8],
    budget: &StateBudget,
    reserved_clone_bytes: usize,
) -> Result<VerifiedRecoveryTransition, AppliedError> {
    validate_empty_recovery_envelope_bytes(bytes).map_err(AppliedError::Recovery)?;
    let envelope =
        decode_compact_recovery_envelope(bytes, budget).map_err(AppliedError::Recovery)?;
    if !envelope.execution.transactions.is_empty() || !envelope.lookahead.transactions.is_empty() {
        return Err(AppliedError::PayloadLimit);
    }
    prepare_development_recovery(
        &parent.recovery,
        Arc::new(envelope),
        budget,
        reserved_clone_bytes,
    )
    .map_err(AppliedError::Recovery)
}
