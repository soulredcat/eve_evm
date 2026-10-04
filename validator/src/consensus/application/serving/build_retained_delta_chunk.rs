// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    charge_delta_serving_work::charge_delta_serving_work,
    current_delta_serving_view::current_delta_serving_view,
    estimate_delta_commit_charge::estimate_delta_commit_charge,
    types::{DeltaServingBudget, DeltaServingError},
};
use crate::consensus::application::ConsensusApplication;
use eve_state::{
    Bytes, StateDeltaChunk, StateDeltaPayload, StateDeltaRequest, encode_state_delta_chunk,
    encode_state_delta_payload, hash_state_delta_bytes, measure_complete_state_bytes,
    measure_state_delta_execution_bytes, measure_state_delta_payload, project_state_journal,
};
use eve_storage::state::{
    capture_state_snapshot, read_snapshot_commit, snapshot_commit_encoded_length, state_reader,
};

pub(super) fn build_retained_delta_chunk(
    application: &ConsensusApplication,
    request: &StateDeltaRequest,
    budget: DeltaServingBudget,
) -> Result<Vec<u8>, DeltaServingError> {
    let mut used = request.maximum_chunk_bytes as usize;
    used = used
        .checked_mul(3)
        .and_then(|bytes| bytes.checked_add(131_072))
        .ok_or(DeltaServingError::ResourceLimit)?;
    charge_delta_serving_work(&mut used, 0, budget.maximum_working_bytes)?;
    let current = current_delta_serving_view(application, budget, &mut used)?;
    if request.target_height == 0 || request.target_height > current.commit().target.height {
        return Err(DeltaServingError::Gap);
    }
    let logical = &application.config.logical_budget;
    let head_upper = measure_complete_state_bytes(&current.commit().state, logical)
        .map_err(|_| DeltaServingError::NotReady)?
        .checked_add(
            measure_state_delta_execution_bytes(&current.commit().block, logical)
                .map_err(|_| DeltaServingError::NotReady)?,
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
    let parent_height = request
        .target_height
        .checked_sub(1)
        .ok_or(DeltaServingError::Gap)?;
    let parent_bytes = snapshot_commit_encoded_length(&snapshot, parent_height)
        .map_err(|_| DeltaServingError::NotReady)?
        .ok_or(DeltaServingError::Gap)?;
    charge_delta_serving_work(
        &mut used,
        estimate_delta_commit_charge(parent_bytes)?,
        budget.maximum_working_bytes,
    )?;
    let parent = read_snapshot_commit(&snapshot, parent_height)
        .map_err(|_| DeltaServingError::NotReady)?
        .ok_or(DeltaServingError::Gap)?;
    let historical_target = if request.target_height == current.commit().target.height {
        None
    } else {
        let target_bytes = snapshot_commit_encoded_length(&snapshot, request.target_height)
            .map_err(|_| DeltaServingError::NotReady)?
            .ok_or(DeltaServingError::Gap)?;
        charge_delta_serving_work(
            &mut used,
            estimate_delta_commit_charge(target_bytes)?,
            budget.maximum_working_bytes,
        )?;
        Some(
            read_snapshot_commit(&snapshot, request.target_height)
                .map_err(|_| DeltaServingError::NotReady)?
                .ok_or(DeltaServingError::Gap)?,
        )
    };
    let target = historical_target.as_ref().unwrap_or(current.commit());
    if parent.target != request.parent {
        return Err(DeltaServingError::Gap);
    }
    if target.parent.as_ref() != Some(&parent.target)
        || target.target.height != request.target_height
    {
        return Err(DeltaServingError::NotReady);
    }
    let journal = project_state_journal(
        &parent.state,
        &parent.target,
        &target.state,
        request.target_height,
        logical,
    )
    .map_err(|_| DeltaServingError::ResourceLimit)?;
    let payload = StateDeltaPayload {
        journal,
        execution: target.block.clone(),
    };
    let size = measure_state_delta_payload(&payload, logical)
        .map_err(|_| DeltaServingError::ResourceLimit)?;
    if size > budget.maximum_delta_bytes {
        return Err(DeltaServingError::ResourceLimit);
    }
    let offset = usize::try_from(request.offset).map_err(|_| DeltaServingError::Gap)?;
    if offset >= size {
        return Err(DeltaServingError::Gap);
    }
    charge_delta_serving_work(
        &mut used,
        size.checked_mul(4)
            .ok_or(DeltaServingError::ResourceLimit)?,
        budget.maximum_working_bytes,
    )?;
    let body = encode_state_delta_payload(&payload, logical)
        .map_err(|_| DeltaServingError::ResourceLimit)?;
    let end = offset
        .checked_add(request.maximum_chunk_bytes as usize)
        .ok_or(DeltaServingError::Gap)?
        .min(body.len());
    let chunk = StateDeltaChunk {
        parent: parent.target.clone(),
        target: target.target.clone(),
        durable_tip: snapshot.version().clone(),
        body_sha256: hash_state_delta_bytes(&body),
        total_length: body.len() as u64,
        offset: request.offset,
        data: Bytes::copy_from_slice(&body[offset..end]),
    };
    encode_state_delta_chunk(&chunk).map_err(|_| DeltaServingError::ResourceLimit)
}
