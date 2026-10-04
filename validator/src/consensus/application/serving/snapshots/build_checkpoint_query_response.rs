// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    charge_delta_serving_work::charge_delta_serving_work,
    current_delta_serving_view::current_delta_serving_view,
    estimate_delta_commit_charge::estimate_delta_commit_charge, types::DeltaServingError,
};
use super::{
    build_checkpoint_message_response::build_checkpoint_message_response,
    snapshot_message_limits::snapshot_message_limits, snapshot_view_budget::snapshot_view_budget,
    types::SnapshotServingBudget,
};
use crate::consensus::application::ConsensusApplication;
use eve_state::{measure_complete_state_bytes, measure_state_delta_execution_bytes};
use eve_storage::{
    checkpoints::messages::CheckpointRequest,
    state::{
        capture_state_snapshot, read_snapshot_commit, snapshot_commit_encoded_length, state_reader,
    },
};
pub(super) fn build_checkpoint_query_response(
    application: &ConsensusApplication,
    request: &CheckpointRequest,
    budget: SnapshotServingBudget,
) -> Result<Vec<u8>, DeltaServingError> {
    let mut used = 2 * 1_048_576;
    charge_delta_serving_work(
        &mut used,
        budget
            .maximum_manifest_bytes
            .checked_mul(4)
            .ok_or(DeltaServingError::ResourceLimit)?,
        budget.maximum_working_bytes,
    )?;
    let current = current_delta_serving_view(application, snapshot_view_budget(budget), &mut used)?;
    if request.height > current.commit().target.height {
        return Err(DeltaServingError::Gap);
    }
    let logical = &application.config.logical_budget;
    let head_upper = measure_complete_state_bytes(&current.commit().state, logical)
        .map_err(|_| DeltaServingError::ResourceLimit)?
        .checked_add(
            measure_state_delta_execution_bytes(&current.commit().block, logical)
                .map_err(|_| DeltaServingError::ResourceLimit)?,
        )
        .and_then(|bytes| bytes.checked_add(12_288))
        .ok_or(DeltaServingError::ResourceLimit)?;
    charge_delta_serving_work(
        &mut used,
        estimate_delta_commit_charge(head_upper)?,
        budget.maximum_working_bytes,
    )?;
    let reader = state_reader(&application.repository);
    let snapshot = capture_state_snapshot(&reader).map_err(|_| DeltaServingError::NotReady)?;
    if snapshot.version() != &current.commit().target {
        return Err(DeltaServingError::NotReady);
    }
    let historical = if request.height == current.commit().target.height {
        None
    } else {
        let encoded = snapshot_commit_encoded_length(&snapshot, request.height)
            .map_err(|_| DeltaServingError::NotReady)?
            .ok_or(DeltaServingError::Gap)?;
        charge_delta_serving_work(
            &mut used,
            estimate_delta_commit_charge(encoded)?,
            budget.maximum_working_bytes,
        )?;
        Some(
            read_snapshot_commit(&snapshot, request.height)
                .map_err(|_| DeltaServingError::NotReady)?
                .ok_or(DeltaServingError::Gap)?,
        )
    };
    let target = historical.as_ref().unwrap_or(current.commit());
    if target.target.height != request.height
        || target.target.identity != application.config.genesis.target.identity
    {
        return Err(DeltaServingError::NotReady);
    }
    build_checkpoint_message_response(
        target,
        snapshot.version(),
        request,
        snapshot_message_limits(*logical, budget),
        &mut used,
        budget.maximum_working_bytes,
    )
}
