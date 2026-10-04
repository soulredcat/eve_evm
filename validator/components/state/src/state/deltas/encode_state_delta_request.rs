// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    append_delta_field::append_delta_field,
    types::{
        MAXIMUM_STATE_DELTA_CHUNK_BYTES, MAXIMUM_STATE_DELTA_REQUEST_BYTES, REQUEST_DOMAIN,
        StateDeltaError, StateDeltaRequest,
    },
};
use crate::encode_state_version;

pub fn encode_state_delta_request(request: &StateDeltaRequest) -> Result<Vec<u8>, StateDeltaError> {
    if request.maximum_chunk_bytes == 0
        || request.maximum_chunk_bytes as usize > MAXIMUM_STATE_DELTA_CHUNK_BYTES
    {
        return Err(StateDeltaError::BudgetExceeded);
    }
    let parent = encode_state_version(&request.parent).map_err(StateDeltaError::State)?;
    let size = REQUEST_DOMAIN
        .len()
        .checked_add(24)
        .and_then(|bytes| bytes.checked_add(parent.len()))
        .ok_or(StateDeltaError::BudgetExceeded)?;
    if parent.len() > 4_096 || size > MAXIMUM_STATE_DELTA_REQUEST_BYTES {
        return Err(StateDeltaError::BudgetExceeded);
    }
    let mut output = Vec::new();
    output
        .try_reserve_exact(size)
        .map_err(|_| StateDeltaError::AllocationFailed)?;
    output.extend_from_slice(REQUEST_DOMAIN);
    append_delta_field(&mut output, &parent)?;
    output.extend_from_slice(&request.target_height.to_be_bytes());
    output.extend_from_slice(&request.offset.to_be_bytes());
    output.extend_from_slice(&request.maximum_chunk_bytes.to_be_bytes());
    Ok(output)
}
