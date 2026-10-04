// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    encode_state_delta_payload,
    types::{StateDeltaError, StateDeltaPayload, StateDeltaPayloadPreflight},
};
use crate::{decode_block_payload, decode_state_journal};

/// Canonical decode only. Complete reencoding uses additional bounded buffers.
pub fn decode_state_delta_payload(
    preflight: &StateDeltaPayloadPreflight<'_>,
) -> Result<StateDeltaPayload, StateDeltaError> {
    let payload = StateDeltaPayload {
        journal: decode_state_journal(preflight.journal, &preflight.budget)
            .map_err(StateDeltaError::State)?,
        execution: decode_block_payload(preflight.execution, &preflight.budget)
            .map_err(StateDeltaError::State)?,
    };
    if encode_state_delta_payload(&payload, &preflight.budget)? != preflight.bytes {
        return Err(StateDeltaError::NonCanonicalEncoding);
    }
    Ok(payload)
}
