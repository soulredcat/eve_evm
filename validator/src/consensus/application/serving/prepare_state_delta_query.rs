// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    build_retained_delta_chunk::build_retained_delta_chunk,
    classify_delta_request_error::classify_delta_request_error,
    types::{DELTA_QUERY_PATH, DeltaServingBudget, DeltaServingError},
    validate_delta_serving_budget::validate_delta_serving_budget,
};
use crate::consensus::application::ConsensusApplication;
use eve_consensus_comet::wire::tendermint::abci::{RequestQuery, ResponseQuery};
use eve_state::decode_state_delta_request;

pub(super) fn prepare_state_delta_query(
    application: &mut ConsensusApplication,
    input: &RequestQuery,
    budget: DeltaServingBudget,
) -> Result<ResponseQuery, DeltaServingError> {
    validate_delta_serving_budget(budget)?;
    if input.path != DELTA_QUERY_PATH {
        return Err(DeltaServingError::UnsupportedVersion);
    }
    if application.fenced {
        return Err(DeltaServingError::NotReady);
    }
    if input.prove || input.height < 0 {
        return Err(DeltaServingError::MalformedRequest);
    }
    if input.data.len() > budget.maximum_request_bytes {
        return Err(DeltaServingError::ResourceLimit);
    }
    let request = decode_state_delta_request(&input.data).map_err(classify_delta_request_error)?;
    if request.parent.identity != application.config.genesis.target.identity {
        return Err(DeltaServingError::WrongNetwork);
    }
    if request.target_height > i64::MAX as u64
        || (input.height != 0 && input.height as u64 != request.target_height)
        || request.parent.height.checked_add(1) != Some(request.target_height)
    {
        return Err(DeltaServingError::Gap);
    }
    if request.maximum_chunk_bytes as usize > budget.maximum_chunk_bytes {
        return Err(DeltaServingError::ResourceLimit);
    }
    if request.offset >= budget.maximum_delta_bytes as u64 {
        return Err(DeltaServingError::Gap);
    }
    let bytes = build_retained_delta_chunk(application, &request, budget)?;
    Ok(ResponseQuery {
        code: 0,
        codespace: "EVE_RECOVERY".into(),
        height: request.target_height as i64,
        value: bytes,
        ..Default::default()
    })
}
