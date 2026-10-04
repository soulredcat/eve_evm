// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    charge_delta_serving_work::charge_delta_serving_work,
    estimate_delta_commit_charge::estimate_delta_commit_charge,
    types::{DeltaServingBudget, DeltaServingError},
};
use crate::consensus::application::ConsensusApplication;
use eve_storage::state::{ImmutableStateView, read_cached_state_service, read_state_service};
use std::sync::Arc;

pub(super) fn current_delta_serving_view(
    application: &ConsensusApplication,
    budget: DeltaServingBudget,
    used: &mut usize,
) -> Result<Arc<ImmutableStateView>, DeltaServingError> {
    if let Some(view) =
        read_cached_state_service(&application.service).map_err(|_| DeltaServingError::NotReady)?
    {
        return Ok(view);
    }
    // A cold cache has no actual head-size fact available yet. Admit the configured
    // maximum before implicit canonical repository reload; never guess from genesis.
    let charge =
        estimate_delta_commit_charge(application.config.logical_budget.maximum_commit_bytes)?;
    charge_delta_serving_work(used, charge, budget.maximum_working_bytes)?;
    read_state_service(&application.service).map_err(|_| DeltaServingError::NotReady)
}
