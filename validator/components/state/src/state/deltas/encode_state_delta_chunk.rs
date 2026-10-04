// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    append_delta_field::append_delta_field,
    hash_state_delta_bytes,
    types::{CHUNK_DOMAIN, StateDeltaChunk, StateDeltaError},
    validate_state_delta_chunk::validate_state_delta_chunk,
};
use crate::encode_state_version;

pub fn encode_state_delta_chunk(chunk: &StateDeltaChunk) -> Result<Vec<u8>, StateDeltaError> {
    validate_state_delta_chunk(
        &chunk.parent,
        &chunk.target,
        &chunk.durable_tip,
        chunk.total_length,
        chunk.offset,
        chunk.data.len(),
    )?;
    let versions = [&chunk.parent, &chunk.target, &chunk.durable_tip]
        .into_iter()
        .map(|version| encode_state_version(version).map_err(StateDeltaError::State))
        .collect::<Result<Vec<_>, _>>()?;
    let mut size = CHUNK_DOMAIN
        .len()
        .checked_add(32 + 8 + 8 + 4 + 32)
        .ok_or(StateDeltaError::BudgetExceeded)?;
    for version in &versions {
        if version.len() > 4_096 {
            return Err(StateDeltaError::BudgetExceeded);
        }
        size = size
            .checked_add(4 + version.len())
            .ok_or(StateDeltaError::BudgetExceeded)?;
    }
    size = size
        .checked_add(chunk.data.len())
        .ok_or(StateDeltaError::BudgetExceeded)?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(size)
        .map_err(|_| StateDeltaError::AllocationFailed)?;
    output.extend_from_slice(CHUNK_DOMAIN);
    for version in &versions {
        append_delta_field(&mut output, version)?;
    }
    output.extend_from_slice(&chunk.body_sha256);
    output.extend_from_slice(&chunk.total_length.to_be_bytes());
    output.extend_from_slice(&chunk.offset.to_be_bytes());
    append_delta_field(&mut output, &chunk.data)?;
    let checksum = hash_state_delta_bytes(&output);
    output.extend_from_slice(&checksum);
    Ok(output)
}
