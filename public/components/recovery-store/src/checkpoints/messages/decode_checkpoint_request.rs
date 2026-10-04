// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointMessageError, CheckpointRequest, CheckpointRequestKind,
    MAXIMUM_CHECKPOINT_REQUEST_BYTES, encode_checkpoint_request,
    take_checkpoint_bytes::take_checkpoint_bytes, take_checkpoint_field::take_checkpoint_field,
    take_checkpoint_prefix::take_checkpoint_prefix, types::REQUEST_DOMAIN,
    validate_checkpoint_request::validate_checkpoint_request,
    validate_checkpoint_request_fields::validate_checkpoint_request_fields,
};
use eve_state::{decode_state_version, preflight_state_version};
/// Request is bounded to a 4096-byte genesis and fixed fields before its bounded network String decode.
pub fn decode_checkpoint_request(
    bytes: &[u8],
) -> Result<CheckpointRequest, CheckpointMessageError> {
    if bytes.len() > MAXIMUM_CHECKPOINT_REQUEST_BYTES {
        return Err(CheckpointMessageError::BudgetExceeded);
    }
    let mut remaining = bytes;
    let kind = take_checkpoint_prefix(&mut remaining, REQUEST_DOMAIN)?;
    if !(1..=3).contains(&kind) {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    let genesis = take_checkpoint_bytes(&mut remaining, 4_096)?;
    preflight_state_version(genesis).map_err(CheckpointMessageError::State)?;
    let height = u64::from_be_bytes(take_checkpoint_field(&mut remaining)?);
    let kind = match kind {
        1 => CheckpointRequestKind::Manifest {
            chunk_bytes: u32::from_be_bytes(take_checkpoint_field(&mut remaining)?),
        },
        2 => CheckpointRequestKind::Chunk {
            chunk_bytes: u32::from_be_bytes(take_checkpoint_field(&mut remaining)?),
            manifest_id: take_checkpoint_field(&mut remaining)?,
            index: u32::from_be_bytes(take_checkpoint_field(&mut remaining)?),
        },
        _ => CheckpointRequestKind::Execution,
    };
    if !remaining.is_empty() {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    validate_checkpoint_request_fields(height, &kind)?;
    // Validate all fixed resource fields before owned canonical metadata decode.
    let request = CheckpointRequest {
        genesis: decode_state_version(genesis).map_err(CheckpointMessageError::State)?,
        height,
        kind,
    };
    validate_checkpoint_request(&request)?;
    if encode_checkpoint_request(&request)? != bytes {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    Ok(request)
}
