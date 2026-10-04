// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointMessageError, CheckpointMessageLimits, CheckpointResponse,
    append_checkpoint_bytes::append_checkpoint_bytes, preflight_checkpoint_response,
    types::RESPONSE_DOMAIN, validate_checkpoint_message_limits::validate_checkpoint_message_limits,
};
use eve_state::{encode_block_payload, encode_state_version, measure_state_delta_execution_bytes};

/// Caller charges typed input, maintained codec buffers and complete output before invoking.
pub fn encode_checkpoint_response(
    response: &CheckpointResponse,
    limits: &CheckpointMessageLimits,
) -> Result<Vec<u8>, CheckpointMessageError> {
    validate_checkpoint_message_limits(limits)?;
    if let CheckpointResponse::Manifest {
        target,
        durable_tip,
        ..
    } = response
        && (durable_tip.identity != target.identity || durable_tip.height < target.height)
    {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    let (tag, target) = match response {
        CheckpointResponse::Manifest { target, .. } => (1, target),
        CheckpointResponse::Chunk { target, .. } => (2, target),
        CheckpointResponse::Execution { target, .. } => (3, target),
    };
    if target.height == 0 || target.height > i64::MAX as u64 {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    let target = encode_state_version(target).map_err(CheckpointMessageError::State)?;
    let payload_size = match response {
        CheckpointResponse::Manifest { manifest, .. } => {
            if manifest.len() > limits.maximum_manifest_bytes {
                return Err(CheckpointMessageError::BudgetExceeded);
            }
            manifest.len().checked_add(4_096 + 40)
        }
        CheckpointResponse::Chunk { data, .. } => {
            if data.len() > limits.maximum_chunk_bytes {
                return Err(CheckpointMessageError::BudgetExceeded);
            }
            data.len().checked_add(84)
        }
        CheckpointResponse::Execution { block, .. } => Some(
            measure_state_delta_execution_bytes(block, &limits.logical)
                .map_err(|_| CheckpointMessageError::BudgetExceeded)?
                .checked_add(4)
                .ok_or(CheckpointMessageError::ArithmeticOverflow)?,
        ),
    }
    .ok_or(CheckpointMessageError::ArithmeticOverflow)?;
    let capacity = payload_size
        .checked_add(target.len())
        .and_then(|bytes| bytes.checked_add(RESPONSE_DOMAIN.len() + 7))
        .ok_or(CheckpointMessageError::ArithmeticOverflow)?;
    if capacity
        > limits
            .maximum_body_bytes
            .checked_add(16_384)
            .ok_or(CheckpointMessageError::ArithmeticOverflow)?
    {
        return Err(CheckpointMessageError::BudgetExceeded);
    }
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(capacity)
        .map_err(|_| CheckpointMessageError::AllocationFailed)?;
    bytes.extend_from_slice(RESPONSE_DOMAIN);
    bytes.extend_from_slice(&[1, 0, tag]);
    append_checkpoint_bytes(&mut bytes, &target)?;
    match response {
        CheckpointResponse::Manifest {
            durable_tip,
            manifest_id,
            manifest,
            ..
        } => {
            append_checkpoint_bytes(
                &mut bytes,
                &encode_state_version(durable_tip).map_err(CheckpointMessageError::State)?,
            )?;
            bytes.extend_from_slice(manifest_id);
            append_checkpoint_bytes(&mut bytes, manifest)?;
        }
        CheckpointResponse::Chunk {
            manifest_id,
            body_sha256,
            total_length,
            index,
            chunk_bytes,
            data,
            ..
        } => {
            bytes.extend_from_slice(manifest_id);
            bytes.extend_from_slice(body_sha256);
            bytes.extend_from_slice(&total_length.to_be_bytes());
            bytes.extend_from_slice(&index.to_be_bytes());
            bytes.extend_from_slice(&chunk_bytes.to_be_bytes());
            append_checkpoint_bytes(&mut bytes, data)?;
        }
        CheckpointResponse::Execution { block, .. } => {
            append_checkpoint_bytes(
                &mut bytes,
                &encode_block_payload(block, &limits.logical)
                    .map_err(CheckpointMessageError::State)?,
            )?;
        }
    }
    preflight_checkpoint_response(&bytes, limits)?;
    Ok(bytes)
}
