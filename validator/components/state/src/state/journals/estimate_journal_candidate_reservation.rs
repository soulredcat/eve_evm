// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    estimate_operation_bytes::estimate_operation_bytes,
    validate_journal_budget::validate_journal_budget,
};
use crate::{
    CompleteState, JournalOperation, StateBudget, StateError, StateJournal,
    measure_complete_state_bytes,
};

/// Conservative logical clone/root/encoding charge; not an allocator/RSS bound.
/// Count every possible insertion even if later removed or overwritten. Final-state
/// limits remain separate. The caller already owns inputs and bounded system-codec
/// scratch (at most one canonical record) while computing this estimate.
pub fn estimate_journal_candidate_reservation(
    parent: &CompleteState,
    journal: &StateJournal,
    budget: &StateBudget,
) -> Result<usize, StateError> {
    crate::state::validation::validate_state_identity(&parent.identity)?;
    validate_journal_budget(journal, budget)?;
    let mut accounts = parent.accounts.len();
    let mut slots = parent
        .accounts
        .values()
        .try_fold(0_usize, |sum, account| {
            sum.checked_add(account.storage.len())
        })
        .ok_or(StateError::ArithmeticOverflow)?;
    let mut codes = parent.codes.len();
    let mut code_bytes = parent
        .codes
        .values()
        .try_fold(0_usize, |sum, code| sum.checked_add(code.len()))
        .ok_or(StateError::ArithmeticOverflow)?;
    let mut system = parent.system.len();
    let mut history = parent.block_hashes.len();
    let mut growth = 1_024_usize;
    for operation in &journal.operations {
        growth = growth
            .checked_add(estimate_operation_bytes(operation, budget)?)
            .ok_or(StateError::ArithmeticOverflow)?;
        let counter = match operation {
            JournalOperation::PutAccount { .. } => Some(&mut accounts),
            JournalOperation::PutStorage { .. } => Some(&mut slots),
            JournalOperation::PutCode { code, .. } => {
                code_bytes = code_bytes
                    .checked_add(code.len())
                    .ok_or(StateError::ArithmeticOverflow)?;
                Some(&mut codes)
            }
            JournalOperation::PutSystem { .. } => Some(&mut system),
            JournalOperation::SetExecutionBlockHash { .. } => Some(&mut history),
            _ => None,
        };
        if let Some(count) = counter {
            *count = count.checked_add(1).ok_or(StateError::ArithmeticOverflow)?;
        }
    }
    // Extra per-operation framing covers changed nested list headers. Four full
    // content envelopes cover candidate serialization/root scratch and journal
    // encoding intermediates; map/node allowances include temporary growth.
    let framing = journal
        .operations
        .len()
        .checked_mul(64)
        .ok_or(StateError::ArithmeticOverflow)?;
    let serialized = measure_complete_state_bytes(parent, budget)?
        .checked_add(growth)
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
