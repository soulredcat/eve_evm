// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::{
    resources::{estimate_master_import_bytes, reserve_master_bytes},
    types::{PreparationFailure, RetainedImport},
};
use eve_finality_verifier::{
    ImportedState, decode_logical_import_wire, imported_state_commit, into_imported_state,
    logical_import_wire_stats, preflight_logical_import_wire, prepare_authenticated_import,
};
use eve_state::{
    BOUNDED_STATE_CODEC_SCRATCH_BYTES, StateBudget, estimate_journal_wire_candidate_reservation,
};
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Canonical borrowed preflight precedes every covered decode/candidate allocation.
pub(in crate::sync) fn prepare_master_import(
    parent: &ImportedState,
    bytes: &[u8],
    budget: &StateBudget,
    pool: &Arc<Semaphore>,
) -> Result<RetainedImport, PreparationFailure> {
    use PreparationFailure::{Invalid, Resource};
    let scratch =
        reserve_master_bytes(pool, BOUNDED_STATE_CODEC_SCRATCH_BYTES).map_err(Resource)?;
    let preflight = preflight_logical_import_wire(bytes, budget)
        .map_err(|error| Invalid(anyhow::anyhow!("MASTER_IMPORT_PREFLIGHT: {error:?}")))?;
    let stats = logical_import_wire_stats(&preflight);
    let candidate = estimate_journal_wire_candidate_reservation(
        &imported_state_commit(parent).state,
        &stats.journal,
        budget,
    )
    .map_err(|error| Invalid(anyhow::anyhow!("MASTER_IMPORT_CANDIDATE: {error:?}")))?;
    let charge = estimate_master_import_bytes(stats, candidate).map_err(Resource)?;
    let lease = reserve_master_bytes(pool, charge).map_err(Resource)?;
    let input = decode_logical_import_wire(&preflight)
        .map_err(|error| Invalid(anyhow::anyhow!("MASTER_IMPORT_DECODE: {error:?}")))?;
    let transition = prepare_authenticated_import(parent, Arc::new(input), budget, candidate)
        .map_err(|error| Invalid(anyhow::anyhow!("MASTER_IMPORT_AUTHENTICATION: {error:?}")))?;
    let state = into_imported_state(transition);
    drop(scratch);
    Ok(RetainedImport {
        state,
        _lease: lease,
    })
}
