// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    append_delta_field::append_delta_field,
    measure_state_delta_payload,
    types::{PAYLOAD_DOMAIN, StateDeltaError, StateDeltaPayload},
};
use crate::{StateBudget, encode_block_payload, encode_state_journal};

/// Large components/output require caller charges; exact size is admitted first.
pub fn encode_state_delta_payload(
    payload: &StateDeltaPayload,
    budget: &StateBudget,
) -> Result<Vec<u8>, StateDeltaError> {
    let size = measure_state_delta_payload(payload, budget)?;
    let journal = encode_state_journal(&payload.journal, budget).map_err(StateDeltaError::State)?;
    let execution =
        encode_block_payload(&payload.execution, budget).map_err(StateDeltaError::State)?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(size)
        .map_err(|_| StateDeltaError::AllocationFailed)?;
    output.extend_from_slice(PAYLOAD_DOMAIN);
    append_delta_field(&mut output, &journal)?;
    append_delta_field(&mut output, &execution)?;
    if output.len() != size {
        return Err(StateDeltaError::NonCanonicalEncoding);
    }
    Ok(output)
}
