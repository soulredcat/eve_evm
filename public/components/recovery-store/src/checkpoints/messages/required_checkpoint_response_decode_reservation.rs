// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointMessageError, CheckpointResponsePreflight};
/// Logical allowance only; retain the real caller lease through decode/reencoding and all owned output.
pub fn required_checkpoint_response_decode_reservation(
    preflight: &CheckpointResponsePreflight<'_>,
) -> Result<usize, CheckpointMessageError> {
    preflight
        .stats
        .encoded_bytes
        .checked_mul(128)
        .and_then(|bytes| bytes.checked_add(2 * 1_048_576))
        .and_then(|bytes| bytes.checked_add(preflight.stats.target_network_bytes.checked_mul(4)?))
        .and_then(|bytes| bytes.checked_add(preflight.stats.tip_network_bytes.checked_mul(4)?))
        .ok_or(CheckpointMessageError::ArithmeticOverflow)
}
