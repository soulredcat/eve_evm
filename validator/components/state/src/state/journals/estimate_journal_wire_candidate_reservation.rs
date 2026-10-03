// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    calculate_journal_candidate_reservation::calculate_journal_candidate_reservation,
    check_journal_decode_counts::check_journal_decode_counts,
    journal_candidate_growth::JournalCandidateGrowth,
};
use crate::{CompleteState, JournalDecodePreflight, StateBudget, StateError};

/// Same logical peak charge as the typed estimator, before allocating decoded data.
/// Compute preflight from the exact retained immutable journal bytes in the caller.
/// Publicly constructible statistics authenticate no state or reservation lease.
/// Inputs, decoded journal buffers and bounded codec scratch are separate charges;
/// this estimate does not guarantee allocator/RSS bounds or final-state validity.
pub fn estimate_journal_wire_candidate_reservation(
    parent: &CompleteState,
    preflight: &JournalDecodePreflight,
    budget: &StateBudget,
) -> Result<usize, StateError> {
    crate::state::validation::validate_state_identity(&parent.identity)?;
    check_journal_decode_counts(preflight, budget)?;
    let growth = JournalCandidateGrowth {
        operation_count: preflight.operation_count,
        accounts: preflight.account_operations,
        slots: preflight.storage_operations,
        codes: preflight.code_operations,
        code_bytes: preflight.code_bytes,
        system: preflight.system_operations,
        history: preflight.execution_hash_operations,
        journal_bytes: preflight.conservative_journal_bytes,
    };
    calculate_journal_candidate_reservation(parent, &growth, budget)
}
