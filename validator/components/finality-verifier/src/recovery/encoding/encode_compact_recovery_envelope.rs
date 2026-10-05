// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    append_length_prefixed::append_length_prefixed,
    encode_native_data_frame::encode_native_data_frame, encode_native_frame::encode_native_frame,
};
use crate::recovery::{
    CompactRecoveryEnvelopeV1, RecoveryError,
    bounds::{
        measure_recovery_envelope_bytes::measure_recovery_envelope_bytes, types::RECOVERY_DOMAIN,
    },
};
use eve_state::{StateBudget, encode_block_payload, encode_state_version};

/// Canonical recovery bytes bind source content; they are not a finality capability.
pub fn encode_compact_recovery_envelope(
    envelope: &CompactRecoveryEnvelopeV1,
    budget: &StateBudget,
) -> Result<Vec<u8>, RecoveryError> {
    let size = measure_recovery_envelope_bytes(envelope)?;
    let execution =
        encode_block_payload(&envelope.execution, budget).map_err(RecoveryError::State)?;
    let mut output = Vec::with_capacity(size);
    output.extend_from_slice(RECOVERY_DOMAIN);
    append_length_prefixed(
        &mut output,
        &encode_state_version(&envelope.parent).map_err(RecoveryError::State)?,
    )?;
    append_length_prefixed(
        &mut output,
        &encode_state_version(&envelope.expected).map_err(RecoveryError::State)?,
    )?;
    append_length_prefixed(&mut output, &execution)?;
    append_length_prefixed(&mut output, &encode_native_frame(&envelope.finalized)?)?;
    append_length_prefixed(&mut output, &encode_native_data_frame(&envelope.lookahead)?)?;
    if output.len() != size {
        return Err(RecoveryError::NonCanonicalEncoding);
    }
    Ok(output)
}
