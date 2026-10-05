// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CompleteExecutionError;
use eve_state::StateError;

/// One canonical cost model for actual parent state and operator-budget ceilings.
pub(super) fn calculate_clone_reservation(
    accounts: usize,
    slots: usize,
    code_bytes: usize,
    codes: usize,
    block_hashes: usize,
) -> Result<usize, CompleteExecutionError> {
    let scaffolding = codes.checked_add(block_hashes);
    [
        (Some(accounts), 1_024),
        (Some(slots), 512),
        (Some(code_bytes), 8),
        (scaffolding, 512),
    ]
    .into_iter()
    .try_fold(2_097_152_usize, |sum, (count, multiplier)| {
        sum.checked_add(count?.checked_mul(multiplier)?)
    })
    .ok_or(CompleteExecutionError::State(
        StateError::ArithmeticOverflow,
    ))
}
