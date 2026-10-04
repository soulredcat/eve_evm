// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{
    MAXIMUM_STATE_DELTA_CHUNK_BYTES, MAXIMUM_STATE_DELTA_PAYLOAD_BYTES, StateDeltaError,
};
use crate::StateVersion;

pub(super) fn validate_state_delta_chunk(
    parent: &StateVersion,
    target: &StateVersion,
    durable_tip: &StateVersion,
    total_length: u64,
    offset: u64,
    data_length: usize,
) -> Result<(), StateDeltaError> {
    if parent.identity != target.identity
        || target.identity != durable_tip.identity
        || parent.height.checked_add(1) != Some(target.height)
        || durable_tip.height < target.height
        || total_length == 0
        || total_length > MAXIMUM_STATE_DELTA_PAYLOAD_BYTES as u64
        || data_length == 0
        || data_length > MAXIMUM_STATE_DELTA_CHUNK_BYTES
        || offset
            .checked_add(data_length as u64)
            .is_none_or(|end| end > total_length)
    {
        return Err(StateDeltaError::BudgetExceeded);
    }
    Ok(())
}
