// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Retained untrusted delta serving; no votes, commits, signatures or master dependency.

mod application_delta_serving_budget;
mod build_retained_delta_chunk;
mod charge_delta_serving_work;
mod classify_delta_request_error;
mod current_delta_serving_view;
mod delta_serving_error_response;
mod development_delta_serving_budget;
mod estimate_delta_commit_charge;
mod prepare_state_delta_query;
mod serve_state_delta_query;
mod snapshots;
mod types;
mod validate_delta_serving_budget;

pub(in crate::consensus) use application_delta_serving_budget::application_delta_serving_budget;
pub(in crate::consensus) use development_delta_serving_budget::development_delta_serving_budget;
pub(in crate::consensus) use serve_state_delta_query::serve_state_delta_query;
pub(in crate::consensus::application) use snapshots::validate_snapshot_serving_budget;
pub(in crate::consensus) use snapshots::{
    SnapshotServingBudget, application_snapshot_serving_budget,
    development_snapshot_serving_budget, serve_checkpoint_query,
};
pub(in crate::consensus) use types::DeltaServingBudget;
pub(in crate::consensus::application) use validate_delta_serving_budget::validate_delta_serving_budget;

#[cfg(test)]
mod tests;
