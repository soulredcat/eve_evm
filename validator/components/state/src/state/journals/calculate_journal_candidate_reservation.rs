// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::journal_candidate_growth::JournalCandidateGrowth;
use crate::{CompleteState, StateBudget, StateError, measure_complete_state_bytes};

/// Shared typed/wire logical charge. Inputs/decoder scratch and RSS are separate.
pub(crate) fn calculate_journal_candidate_reservation(
    parent: &CompleteState,
    growth: &JournalCandidateGrowth,
    budget: &StateBudget,
) -> Result<usize, StateError> {
    let accounts = parent
        .accounts
        .len()
        .checked_add(growth.accounts)
        .ok_or(StateError::ArithmeticOverflow)?;
    let slots = parent
        .accounts
        .values()
        .try_fold(0_usize, |sum, account| {
            sum.checked_add(account.storage.len())
        })
        .and_then(|count| count.checked_add(growth.slots))
        .ok_or(StateError::ArithmeticOverflow)?;
    let codes = parent
        .codes
        .len()
        .checked_add(growth.codes)
        .ok_or(StateError::ArithmeticOverflow)?;
    let code_bytes = parent
        .codes
        .values()
        .try_fold(0_usize, |sum, code| sum.checked_add(code.len()))
        .and_then(|count| count.checked_add(growth.code_bytes))
        .ok_or(StateError::ArithmeticOverflow)?;
    let system = parent
        .system
        .len()
        .checked_add(growth.system)
        .ok_or(StateError::ArithmeticOverflow)?;
    let history = parent
        .block_hashes
        .len()
        .checked_add(growth.history)
        .ok_or(StateError::ArithmeticOverflow)?;
    // Preserve the existing framing/envelope/map-node charge without final-state
    // cardinality checks: repeated or later-deleted writes still consume peak work.
    let framing = growth
        .operation_count
        .checked_mul(64)
        .ok_or(StateError::ArithmeticOverflow)?;
    let serialized = measure_complete_state_bytes(parent, budget)?
        .checked_add(growth.journal_bytes)
        .and_then(|bytes| bytes.checked_add(framing))
        .ok_or(StateError::ArithmeticOverflow)?;
    [
        (accounts, 512),
        (slots, 256),
        (codes, 256),
        (code_bytes, 4),
        (system, 512),
        (history, 128),
        (serialized, 4),
    ]
    .into_iter()
    .try_fold(2_097_152_usize, |sum, (count, factor)| {
        sum.checked_add(count.checked_mul(factor)?)
    })
    .ok_or(StateError::ArithmeticOverflow)
}
