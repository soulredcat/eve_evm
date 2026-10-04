// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    measure_state_delta_execution_bytes,
    types::{
        MAXIMUM_STATE_DELTA_PAYLOAD_BYTES, PAYLOAD_DOMAIN, StateDeltaError, StateDeltaPayload,
    },
};
use crate::{StateBudget, measure_state_journal_bytes};

pub fn measure_state_delta_payload(
    payload: &StateDeltaPayload,
    budget: &StateBudget,
) -> Result<usize, StateDeltaError> {
    let journal =
        measure_state_journal_bytes(&payload.journal, budget).map_err(StateDeltaError::State)?;
    if journal > 8_388_608 {
        return Err(StateDeltaError::BudgetExceeded);
    }
    let execution = measure_state_delta_execution_bytes(&payload.execution, budget)?;
    let total = PAYLOAD_DOMAIN
        .len()
        .checked_add(8)
        .and_then(|bytes| bytes.checked_add(journal))
        .and_then(|bytes| bytes.checked_add(execution))
        .ok_or(StateDeltaError::BudgetExceeded)?;
    if total > MAXIMUM_STATE_DELTA_PAYLOAD_BYTES {
        return Err(StateDeltaError::BudgetExceeded);
    }
    Ok(total)
}
