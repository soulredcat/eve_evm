// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! One captured durable state view supplies untrusted snapshot content; no votes, signatures or master dependency.
mod application_snapshot_serving_budget;
mod build_checkpoint_message_response;
mod build_checkpoint_query_response;
mod classify_checkpoint_message_error;
mod development_snapshot_serving_budget;
mod prepare_checkpoint_query;
mod serve_checkpoint_query;
mod snapshot_message_limits;
mod snapshot_view_budget;
mod types;
mod validate_snapshot_serving_budget;
pub(in crate::consensus) use application_snapshot_serving_budget::application_snapshot_serving_budget;
pub(in crate::consensus) use development_snapshot_serving_budget::development_snapshot_serving_budget;
pub(in crate::consensus) use serve_checkpoint_query::serve_checkpoint_query;
pub(in crate::consensus) use types::SnapshotServingBudget;
pub(in crate::consensus::application) use validate_snapshot_serving_budget::validate_snapshot_serving_budget;
#[cfg(test)]
mod tests;
