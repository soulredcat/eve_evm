// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    calculate_journal_candidate_reservation::calculate_journal_candidate_reservation,
    estimate_operation_bytes::estimate_operation_bytes,
    journal_candidate_growth::JournalCandidateGrowth,
    validate_journal_budget::validate_journal_budget,
};
use crate::{CompleteState, JournalOperation, StateBudget, StateError, StateJournal};

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
    let mut growth = JournalCandidateGrowth {
        operation_count: journal.operations.len(),
        journal_bytes: 1_024,
        ..JournalCandidateGrowth::default()
    };
    for operation in &journal.operations {
        growth.journal_bytes = growth
            .journal_bytes
            .checked_add(estimate_operation_bytes(operation, budget)?)
            .ok_or(StateError::ArithmeticOverflow)?;
        let counter = match operation {
            JournalOperation::PutAccount { .. } => Some(&mut growth.accounts),
            JournalOperation::PutStorage { .. } => Some(&mut growth.slots),
            JournalOperation::PutCode { code, .. } => {
                growth.code_bytes = growth
                    .code_bytes
                    .checked_add(code.len())
                    .ok_or(StateError::ArithmeticOverflow)?;
                Some(&mut growth.codes)
            }
            JournalOperation::PutSystem { .. } => Some(&mut growth.system),
            JournalOperation::SetExecutionBlockHash { .. } => Some(&mut growth.history),
            _ => None,
        };
        if let Some(count) = counter {
            *count = count.checked_add(1).ok_or(StateError::ArithmeticOverflow)?;
        }
    }
    calculate_journal_candidate_reservation(parent, &growth, budget)
}
