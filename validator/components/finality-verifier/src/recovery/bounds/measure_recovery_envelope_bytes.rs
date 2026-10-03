// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    measure_execution_payload_bytes::measure_execution_payload_bytes,
    measure_native_frame_bytes::measure_native_frame_bytes,
    measure_transaction_list_bytes::measure_transaction_list_bytes,
    types::{MAXIMUM_RECOVERY_BYTES, MAXIMUM_VERSION_BYTES, RECOVERY_DOMAIN},
};
use crate::recovery::{CompactRecoveryEnvelopeV1, RecoveryError};
use eve_state::encode_state_version;

pub(crate) fn measure_recovery_envelope_bytes(
    envelope: &CompactRecoveryEnvelopeV1,
) -> Result<usize, RecoveryError> {
    let parent = encode_state_version(&envelope.parent)
        .map_err(RecoveryError::State)?
        .len();
    let expected = encode_state_version(&envelope.expected)
        .map_err(RecoveryError::State)?
        .len();
    if parent > MAXIMUM_VERSION_BYTES || expected > MAXIMUM_VERSION_BYTES {
        return Err(RecoveryError::BudgetExceeded);
    }
    let execution = measure_execution_payload_bytes(&envelope.execution)?;
    let finalized = measure_native_frame_bytes(&envelope.finalized)?;
    let lookahead = 4
        + measure_native_frame_bytes(&envelope.lookahead.frame)?
        + measure_transaction_list_bytes(&envelope.lookahead.transactions)?;
    let total = RECOVERY_DOMAIN.len() + 20 + parent + expected + execution + finalized + lookahead;
    if total > MAXIMUM_RECOVERY_BYTES {
        return Err(RecoveryError::BudgetExceeded);
    }
    Ok(total)
}
