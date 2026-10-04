// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    scan_state_delta_execution::scan_state_delta_execution,
    take_delta_field::take_delta_field,
    types::{
        MAXIMUM_EXECUTION_BYTES, MAXIMUM_STATE_DELTA_PAYLOAD_BYTES, PAYLOAD_DOMAIN,
        StateDeltaError, StateDeltaPayloadPreflight, StateDeltaPayloadStats,
    },
};
use crate::{StateBudget, preflight_state_journal};

pub fn preflight_state_delta_payload<'a>(
    bytes: &'a [u8],
    budget: &StateBudget,
) -> Result<StateDeltaPayloadPreflight<'a>, StateDeltaError> {
    if bytes.len() > MAXIMUM_STATE_DELTA_PAYLOAD_BYTES {
        return Err(StateDeltaError::BudgetExceeded);
    }
    let mut remaining = bytes
        .strip_prefix(PAYLOAD_DOMAIN)
        .ok_or(StateDeltaError::UnsupportedVersion)?;
    let journal = take_delta_field(&mut remaining, budget.maximum_journal_bytes.min(8_388_608))?;
    let execution = take_delta_field(
        &mut remaining,
        budget.maximum_commit_bytes.min(MAXIMUM_EXECUTION_BYTES),
    )?;
    if !remaining.is_empty() {
        return Err(StateDeltaError::NonCanonicalEncoding);
    }
    let stats = StateDeltaPayloadStats {
        encoded_bytes: bytes.len(),
        journal: preflight_state_journal(journal, budget).map_err(StateDeltaError::State)?,
        execution: scan_state_delta_execution(execution, budget)?,
    };
    Ok(StateDeltaPayloadPreflight {
        bytes,
        budget: *budget,
        journal,
        execution,
        stats,
    })
}
