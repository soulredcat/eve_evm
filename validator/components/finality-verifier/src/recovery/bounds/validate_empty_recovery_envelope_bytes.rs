// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    scan_execution_payload::scan_execution_payload,
    scan_native_data_frame::scan_native_data_frame,
    scan_native_frame::scan_native_frame,
    types::{MAXIMUM_EXECUTION_HEADER_BYTES, MAXIMUM_NATIVE_FRAME_BYTES, MAXIMUM_RECOVERY_BYTES},
};
use crate::recovery::{
    RecoveryError,
    decoding::{
        slice_compact_recovery_envelope::slice_compact_recovery_envelope,
        take_bounded_rlp_payload::take_bounded_rlp_payload,
        take_length_prefixed::take_length_prefixed, take_u32::take_u32,
    },
};

/// Borrowed admission for the temporary empty-block public application capability.
/// A nonempty transaction list exceeds this service's zero-transaction budget;
/// canonical recovery decoding and replay continue to accept bounded transactions.
/// Success requires subsequent canonical state decoding and finality verification.
pub fn validate_empty_recovery_envelope_bytes(bytes: &[u8]) -> Result<(), RecoveryError> {
    let fields = slice_compact_recovery_envelope(bytes)?;
    scan_execution_payload(fields.execution)?;
    scan_native_frame(fields.finalized)?;
    scan_native_data_frame(fields.lookahead)?;

    let mut execution = fields.execution;
    let mut block = take_bounded_rlp_payload(&mut execution, true, MAXIMUM_RECOVERY_BYTES)?;
    take_bounded_rlp_payload(&mut block, true, MAXIMUM_EXECUTION_HEADER_BYTES)?;
    take_bounded_rlp_payload(&mut block, true, 0)?;

    let mut lookahead = fields.lookahead;
    take_length_prefixed(&mut lookahead, MAXIMUM_NATIVE_FRAME_BYTES)?;
    if take_u32(&mut lookahead)? != 0 {
        return Err(RecoveryError::BudgetExceeded);
    }
    Ok(())
}
