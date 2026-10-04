// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{
    AppliedError,
    resources::{
        EstimatedWorkingPool, estimate_import_charge, reserve_estimated_working,
        split_estimated_working,
    },
    state::AppliedState,
    types::ChargedAppliedState,
};
use eve_finality_verifier::{
    ImportedState, decode_authenticated_import_wire, decode_logical_import_wire, import_wire_stats,
    into_imported_state, logical_import_wire_stats, preflight_authenticated_import_wire,
    preflight_logical_import_wire, prepare_authenticated_import,
};
use eve_state::{
    BOUNDED_STATE_CODEC_SCRATCH_BYTES, StateBudget, estimate_journal_wire_candidate_reservation,
};
use std::sync::Arc;

/// Both sealed wire transports use the same actual candidate/import authority and retained charge.
pub(in crate::sync::applied) fn prepare_charged_import(
    parent: &ImportedState,
    bytes: &[u8],
    budget: &StateBudget,
    working: &Arc<EstimatedWorkingPool>,
    logical_v2: bool,
) -> Result<Arc<ChargedAppliedState>, AppliedError> {
    let scratch = reserve_estimated_working(working, BOUNDED_STATE_CODEC_SCRATCH_BYTES)?;
    let compact = if logical_v2 {
        None
    } else {
        Some(preflight_authenticated_import_wire(bytes, budget).map_err(AppliedError::ImportWire)?)
    };
    let logical = if logical_v2 {
        Some(preflight_logical_import_wire(bytes, budget).map_err(AppliedError::ImportWire)?)
    } else {
        None
    };
    let stats = match (&compact, &logical) {
        (Some(preflight), None) => import_wire_stats(preflight),
        (None, Some(preflight)) => logical_import_wire_stats(preflight),
        _ => return Err(AppliedError::WrongMode),
    };
    let candidate = estimate_journal_wire_candidate_reservation(
        &eve_finality_verifier::imported_state_commit(parent).state,
        &stats.journal,
        budget,
    )
    .map_err(|_| AppliedError::EstimatedCapacity)?;
    let charge = estimate_import_charge(stats, candidate)?;
    let extra = if logical_v2 {
        stats
            .encoded_bytes
            .checked_mul(2)
            .and_then(|bytes| bytes.checked_add(stats.journal.encoded_bytes.checked_mul(2)?))
            .and_then(|bytes| {
                bytes.checked_add(
                    stats
                        .journal
                        .operation_count
                        .checked_mul(2 * std::mem::size_of::<Vec<u8>>())?,
                )
            })
            .ok_or(AppliedError::ArithmeticOverflow)?
    } else {
        0
    };
    let total = charge
        .total
        .checked_add(extra)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let lease = reserve_estimated_working(working, total)?;
    let input = match (&compact, &logical) {
        (Some(preflight), None) => decode_authenticated_import_wire(preflight),
        (None, Some(preflight)) => decode_logical_import_wire(preflight),
        _ => return Err(AppliedError::WrongMode),
    }
    .map_err(AppliedError::ImportWire)?;
    let transition =
        prepare_authenticated_import(parent, Arc::new(input), budget, charge.candidate)
            .map_err(AppliedError::Import)?;
    let state = AppliedState::AuthenticatedImport(into_imported_state(transition));
    let (retained, transient) = split_estimated_working(lease, charge.retained)?;
    let generation = Arc::new(ChargedAppliedState {
        state,
        _lease: retained,
    });
    drop(transient);
    drop(scratch);
    Ok(generation)
}
