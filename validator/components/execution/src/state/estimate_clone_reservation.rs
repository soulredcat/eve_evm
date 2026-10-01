// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CompleteExecutionError;
use eve_state::{CompleteState, StateError};

/// Deterministic conservative reservation for BOTH oracle caches, code analysis,
/// and conversion scaffolding. It is not an allocator/RSS measurement; caller
/// separately accounts retained input views and transactions/receipts.
pub fn estimate_clone_reservation(state: &CompleteState) -> Result<usize, CompleteExecutionError> {
    let overflow = CompleteExecutionError::State(StateError::ArithmeticOverflow);
    let accounts = state
        .accounts
        .len()
        .checked_mul(1_024)
        .ok_or_else(|| overflow.clone())?;
    let slots = state
        .accounts
        .values()
        .try_fold(0_usize, |sum, account| {
            sum.checked_add(account.storage.len())
        })
        .ok_or_else(|| overflow.clone())?
        .checked_mul(512)
        .ok_or_else(|| overflow.clone())?;
    let code = state
        .codes
        .values()
        .try_fold(0_usize, |sum, code| sum.checked_add(code.len()))
        .ok_or_else(|| overflow.clone())?
        .checked_mul(8)
        .ok_or_else(|| overflow.clone())?;
    let scaffolding = state
        .codes
        .len()
        .checked_add(state.block_hashes.len())
        .and_then(|count| count.checked_mul(512))
        .ok_or_else(|| overflow.clone())?;
    [accounts, slots, code, scaffolding]
        .into_iter()
        .try_fold(2_097_152_usize, |sum, value| sum.checked_add(value))
        .ok_or(overflow)
}
