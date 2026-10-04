// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    encode_state_delta_chunk, hash_state_delta_bytes,
    take_delta_field::take_delta_field,
    types::{CHUNK_DOMAIN, MAXIMUM_STATE_DELTA_CHUNK_BYTES, StateDeltaChunk, StateDeltaError},
    validate_state_delta_chunk::validate_state_delta_chunk,
};
use crate::{Bytes, decode_state_version};

pub fn decode_state_delta_chunk(bytes: &[u8]) -> Result<StateDeltaChunk, StateDeltaError> {
    if bytes.len() > MAXIMUM_STATE_DELTA_CHUNK_BYTES + 16_384 || bytes.len() < 32 {
        return Err(StateDeltaError::BudgetExceeded);
    }
    let (body, checksum) = bytes.split_at(bytes.len() - 32);
    if hash_state_delta_bytes(body).as_slice() != checksum {
        return Err(StateDeltaError::ChecksumMismatch);
    }
    let mut remaining = body
        .strip_prefix(CHUNK_DOMAIN)
        .ok_or(StateDeltaError::UnsupportedVersion)?;
    let parent = decode_state_version(take_delta_field(&mut remaining, 4_096)?)
        .map_err(StateDeltaError::State)?;
    let target = decode_state_version(take_delta_field(&mut remaining, 4_096)?)
        .map_err(StateDeltaError::State)?;
    let durable_tip = decode_state_version(take_delta_field(&mut remaining, 4_096)?)
        .map_err(StateDeltaError::State)?;
    let fixed = remaining
        .get(..48)
        .ok_or(StateDeltaError::MalformedEncoding)?;
    let body_sha256 = fixed[..32]
        .try_into()
        .map_err(|_| StateDeltaError::MalformedEncoding)?;
    let total_length = u64::from_be_bytes(
        fixed[32..40]
            .try_into()
            .map_err(|_| StateDeltaError::MalformedEncoding)?,
    );
    let offset = u64::from_be_bytes(
        fixed[40..]
            .try_into()
            .map_err(|_| StateDeltaError::MalformedEncoding)?,
    );
    remaining = &remaining[48..];
    let data = take_delta_field(&mut remaining, MAXIMUM_STATE_DELTA_CHUNK_BYTES)?;
    if !remaining.is_empty() {
        return Err(StateDeltaError::NonCanonicalEncoding);
    }
    validate_state_delta_chunk(
        &parent,
        &target,
        &durable_tip,
        total_length,
        offset,
        data.len(),
    )?;
    let chunk = StateDeltaChunk {
        parent,
        target,
        durable_tip,
        body_sha256,
        total_length,
        offset,
        data: Bytes::copy_from_slice(data),
    };
    if encode_state_delta_chunk(&chunk)? != bytes {
        return Err(StateDeltaError::NonCanonicalEncoding);
    }
    Ok(chunk)
}
