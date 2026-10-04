// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    charge_delta_serving_work::charge_delta_serving_work, types::DeltaServingError,
};
use eve_state::{
    Bytes, StateCommit, StateVersion, encode_state_commit, encode_state_version,
    measure_complete_state_bytes, measure_state_delta_execution_bytes, preflight_state_commit,
};
use eve_storage::checkpoints::{
    checkpoint_manifest_id, checkpoint_manifest_stats, create_checkpoint_manifest,
    messages::{
        CheckpointMessageLimits, CheckpointRequest, CheckpointRequestKind, CheckpointResponse,
        checkpoint_message_storage_limits, encode_checkpoint_response,
    },
    preflight_checkpoint_manifest, required_checkpoint_metadata_reservation,
};

/// One already captured target supplies all output. No I/O, native certificate, voting or state mutation.
pub(super) fn build_checkpoint_message_response(
    target: &StateCommit,
    durable_tip: &StateVersion,
    request: &CheckpointRequest,
    limits: CheckpointMessageLimits,
    used: &mut usize,
    maximum: usize,
) -> Result<Vec<u8>, DeltaServingError> {
    if matches!(request.kind, CheckpointRequestKind::Execution) {
        let response = CheckpointResponse::Execution {
            target: target.target.clone(),
            block: Box::new(target.block.clone()),
        };
        return encode_checkpoint_response(&response, &limits)
            .map_err(|_| DeltaServingError::ResourceLimit);
    }
    let width = match request.kind {
        CheckpointRequestKind::Manifest { chunk_bytes }
        | CheckpointRequestKind::Chunk { chunk_bytes, .. } => chunk_bytes as usize,
        CheckpointRequestKind::Execution => return Err(DeltaServingError::MalformedRequest),
    };
    let storage_limits = checkpoint_message_storage_limits(&limits, width)
        .map_err(|_| DeltaServingError::ResourceLimit)?;
    let metadata = required_checkpoint_metadata_reservation(&storage_limits)
        .map_err(|_| DeltaServingError::ResourceLimit)?;
    charge_delta_serving_work(used, metadata, maximum)?;
    let upper = measure_complete_state_bytes(&target.state, &limits.logical)
        .map_err(|_| DeltaServingError::ResourceLimit)?
        .checked_add(
            measure_state_delta_execution_bytes(&target.block, &limits.logical)
                .map_err(|_| DeltaServingError::ResourceLimit)?,
        )
        .and_then(|bytes| bytes.checked_add(12_288))
        .ok_or(DeltaServingError::ResourceLimit)?;
    if upper > limits.maximum_body_bytes {
        return Err(DeltaServingError::ResourceLimit);
    }
    charge_delta_serving_work(
        used,
        upper
            .checked_mul(4)
            .ok_or(DeltaServingError::ResourceLimit)?,
        maximum,
    )?;
    let body = encode_state_commit(target, &limits.logical)
        .map_err(|_| DeltaServingError::ResourceLimit)?;
    if body.len() > limits.maximum_body_bytes {
        return Err(DeltaServingError::ResourceLimit);
    }
    let state =
        preflight_state_commit(&body, &limits.logical).map_err(|_| DeltaServingError::NotReady)?;
    let manifest = create_checkpoint_manifest(&state, &target.target, &storage_limits, metadata)
        .map_err(|_| DeltaServingError::ResourceLimit)?;
    let version = encode_state_version(&target.target).map_err(|_| DeltaServingError::NotReady)?;
    let preflight = preflight_checkpoint_manifest(&manifest, &version, &storage_limits)
        .map_err(|_| DeltaServingError::NotReady)?;
    let id = checkpoint_manifest_id(&preflight);
    let stats = checkpoint_manifest_stats(&preflight);
    let response = match request.kind {
        CheckpointRequestKind::Manifest { .. } => CheckpointResponse::Manifest {
            target: target.target.clone(),
            durable_tip: Box::new(durable_tip.clone()),
            manifest_id: id,
            manifest,
        },
        CheckpointRequestKind::Chunk {
            manifest_id, index, ..
        } => {
            if manifest_id != id {
                return Err(DeltaServingError::Gap);
            }
            let start = (index as usize)
                .checked_mul(width)
                .ok_or(DeltaServingError::ResourceLimit)?;
            if start >= body.len() {
                return Err(DeltaServingError::Gap);
            }
            let end = start
                .checked_add(width)
                .ok_or(DeltaServingError::ResourceLimit)?
                .min(body.len());
            charge_delta_serving_work(
                used,
                (end - start)
                    .checked_mul(4)
                    .ok_or(DeltaServingError::ResourceLimit)?,
                maximum,
            )?;
            CheckpointResponse::Chunk {
                target: target.target.clone(),
                manifest_id: id,
                body_sha256: stats.body_sha256,
                total_length: body.len() as u64,
                index,
                chunk_bytes: width as u32,
                data: Bytes::copy_from_slice(&body[start..end]),
            }
        }
        CheckpointRequestKind::Execution => return Err(DeltaServingError::MalformedRequest),
    };
    encode_checkpoint_response(&response, &limits).map_err(|_| DeltaServingError::ResourceLimit)
}
