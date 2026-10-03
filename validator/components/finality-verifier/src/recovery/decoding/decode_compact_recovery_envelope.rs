// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_native_data_frame::decode_native_data_frame, decode_native_frame::decode_native_frame,
    slice_compact_recovery_envelope::slice_compact_recovery_envelope,
};
use crate::recovery::{
    CompactRecoveryEnvelopeV1, RecoveryError,
    bounds::{
        scan_execution_payload::scan_execution_payload,
        scan_native_data_frame::scan_native_data_frame, scan_native_frame::scan_native_frame,
        validate_recovery_envelope_bounds::validate_recovery_envelope_bounds,
    },
};
use eve_state::{StateBudget, decode_block_payload, decode_state_version};

pub fn decode_compact_recovery_envelope(
    input: &[u8],
    budget: &StateBudget,
) -> Result<CompactRecoveryEnvelopeV1, RecoveryError> {
    let fields = slice_compact_recovery_envelope(input)?;
    scan_execution_payload(fields.execution)?;
    scan_native_frame(fields.finalized)?;
    scan_native_data_frame(fields.lookahead)?;
    let envelope = CompactRecoveryEnvelopeV1 {
        parent: decode_state_version(fields.parent).map_err(RecoveryError::State)?,
        expected: decode_state_version(fields.expected).map_err(RecoveryError::State)?,
        execution: decode_block_payload(fields.execution, budget).map_err(RecoveryError::State)?,
        finalized: decode_native_frame(fields.finalized)?,
        lookahead: decode_native_data_frame(fields.lookahead)?,
    };
    validate_recovery_envelope_bounds(&envelope)?;
    Ok(envelope)
}
