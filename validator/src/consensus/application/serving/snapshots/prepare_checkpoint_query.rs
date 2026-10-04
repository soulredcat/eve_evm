// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::DeltaServingError;
use super::{
    build_checkpoint_query_response::build_checkpoint_query_response,
    classify_checkpoint_message_error::classify_checkpoint_message_error,
    types::{CHECKPOINT_QUERY_PATH, SnapshotServingBudget},
    validate_snapshot_serving_budget::validate_snapshot_serving_budget,
};
use crate::consensus::application::ConsensusApplication;
use eve_consensus_comet::wire::tendermint::abci::{RequestQuery, ResponseQuery};
use eve_storage::checkpoints::messages::{CheckpointRequestKind, decode_checkpoint_request};
pub(super) fn prepare_checkpoint_query(
    application: &mut ConsensusApplication,
    input: &RequestQuery,
    budget: SnapshotServingBudget,
) -> Result<ResponseQuery, DeltaServingError> {
    validate_snapshot_serving_budget(budget)?;
    if input.path != CHECKPOINT_QUERY_PATH {
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
    let request =
        decode_checkpoint_request(&input.data).map_err(classify_checkpoint_message_error)?;
    if request.genesis != application.config.genesis.target {
        return Err(DeltaServingError::WrongNetwork);
    }
    if input.height != 0 && input.height as u64 != request.height {
        return Err(DeltaServingError::Gap);
    }
    let width = match request.kind {
        CheckpointRequestKind::Manifest { chunk_bytes }
        | CheckpointRequestKind::Chunk { chunk_bytes, .. } => chunk_bytes as usize,
        CheckpointRequestKind::Execution => 0,
    };
    if width > budget.maximum_chunk_bytes {
        return Err(DeltaServingError::ResourceLimit);
    }
    let value = build_checkpoint_query_response(application, &request, budget)?;
    Ok(ResponseQuery {
        code: 0,
        codespace: "EVE_RECOVERY".into(),
        height: request.height as i64,
        value,
        ..Default::default()
    })
}
