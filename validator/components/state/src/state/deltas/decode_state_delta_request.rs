// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    encode_state_delta_request,
    take_delta_field::take_delta_field,
    types::{
        MAXIMUM_STATE_DELTA_REQUEST_BYTES, REQUEST_DOMAIN, StateDeltaError, StateDeltaRequest,
    },
};
use crate::decode_state_version;

pub fn decode_state_delta_request(bytes: &[u8]) -> Result<StateDeltaRequest, StateDeltaError> {
    if bytes.len() > MAXIMUM_STATE_DELTA_REQUEST_BYTES {
        return Err(StateDeltaError::BudgetExceeded);
    }
    let mut remaining = bytes.strip_prefix(REQUEST_DOMAIN).ok_or_else(|| {
        if bytes.starts_with(b"EVE_DELTA_REQ_V") {
            StateDeltaError::UnsupportedVersion
        } else {
            StateDeltaError::MalformedEncoding
        }
    })?;
    let parent = decode_state_version(take_delta_field(&mut remaining, 4_096)?)
        .map_err(StateDeltaError::State)?;
    if remaining.len() != 20 {
        return Err(StateDeltaError::MalformedEncoding);
    }
    let request = StateDeltaRequest {
        parent,
        target_height: u64::from_be_bytes(
            remaining[..8]
                .try_into()
                .map_err(|_| StateDeltaError::MalformedEncoding)?,
        ),
        offset: u64::from_be_bytes(
            remaining[8..16]
                .try_into()
                .map_err(|_| StateDeltaError::MalformedEncoding)?,
        ),
        maximum_chunk_bytes: u32::from_be_bytes(
            remaining[16..]
                .try_into()
                .map_err(|_| StateDeltaError::MalformedEncoding)?,
        ),
    };
    if encode_state_delta_request(&request)? != bytes {
        return Err(StateDeltaError::NonCanonicalEncoding);
    }
    Ok(request)
}
