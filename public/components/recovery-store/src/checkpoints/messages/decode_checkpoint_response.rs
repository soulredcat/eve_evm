// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointMessageError, CheckpointResponse, CheckpointResponseKind,
    CheckpointResponsePreflight, encode_checkpoint_response,
    required_checkpoint_response_decode_reservation,
};
use eve_state::{Bytes, decode_block_payload, decode_state_version};
/// Only sealed same-byte policy decodes. Parsed response data grants no certificate or state authority.
pub fn decode_checkpoint_response(
    preflight: &CheckpointResponsePreflight<'_>,
    reserved: usize,
) -> Result<CheckpointResponse, CheckpointMessageError> {
    if reserved < required_checkpoint_response_decode_reservation(preflight)? {
        return Err(CheckpointMessageError::ReservationTooSmall);
    }
    let target = decode_state_version(preflight.target).map_err(CheckpointMessageError::State)?;
    if target.height == 0 || target.height > i64::MAX as u64 {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    let response = match preflight.kind {
        CheckpointResponseKind::Manifest => {
            let durable_tip = decode_state_version(
                preflight
                    .tip
                    .ok_or(CheckpointMessageError::MalformedEncoding)?,
            )
            .map_err(CheckpointMessageError::State)?;
            if durable_tip.identity != target.identity || durable_tip.height < target.height {
                return Err(CheckpointMessageError::MalformedEncoding);
            }
            CheckpointResponse::Manifest {
                target,
                durable_tip: Box::new(durable_tip),
                manifest_id: preflight
                    .manifest_id
                    .ok_or(CheckpointMessageError::MalformedEncoding)?,
                manifest: preflight.payload.to_vec(),
            }
        }
        CheckpointResponseKind::Chunk => CheckpointResponse::Chunk {
            target,
            manifest_id: preflight
                .manifest_id
                .ok_or(CheckpointMessageError::MalformedEncoding)?,
            body_sha256: preflight
                .body_hash
                .ok_or(CheckpointMessageError::MalformedEncoding)?,
            total_length: preflight.total,
            index: preflight.index,
            chunk_bytes: preflight.width,
            data: Bytes::copy_from_slice(preflight.payload),
        },
        CheckpointResponseKind::Execution => CheckpointResponse::Execution {
            target,
            block: Box::new(
                decode_block_payload(preflight.payload, &preflight.limits.logical)
                    .map_err(CheckpointMessageError::State)?,
            ),
        },
    };
    if encode_checkpoint_response(&response, &preflight.limits)? != preflight.bytes {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    Ok(response)
}
