// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CompleteExecutionError, calculate_clone_reservation::calculate_clone_reservation};
use eve_state::{CompleteState, StateError};

/// Deterministic conservative reservation for BOTH oracle caches, code analysis,
/// and conversion scaffolding. It is not an allocator/RSS measurement; caller
/// separately accounts retained input views and transactions/receipts.
pub fn estimate_clone_reservation(state: &CompleteState) -> Result<usize, CompleteExecutionError> {
    let overflow = CompleteExecutionError::State(StateError::ArithmeticOverflow);
    let slots = state
        .accounts
        .values()
        .try_fold(0_usize, |sum, account| {
            sum.checked_add(account.storage.len())
        })
        .ok_or_else(|| overflow.clone())?;
    let code = state
        .codes
        .values()
        .try_fold(0_usize, |sum, code| sum.checked_add(code.len()))
        .ok_or_else(|| overflow.clone())?;
    calculate_clone_reservation(
        state.accounts.len(),
        slots,
        code,
        state.codes.len(),
        state.block_hashes.len(),
    )
}
