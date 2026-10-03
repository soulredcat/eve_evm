// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{apply_state_journal, estimate_journal_candidate_reservation};
use crate::{CompleteState, StateBudget, StateError, StateJournal, StateVersion};

/// Check the caller's logical candidate charge before any cloned candidate is built.
/// This parameter is not a lease; public must hold its actual accounting reservation.
pub fn apply_state_journal_reserved(
    parent: &CompleteState,
    version: &StateVersion,
    journal: &StateJournal,
    budget: &StateBudget,
    reserved_candidate_bytes: usize,
) -> Result<CompleteState, StateError> {
    if journal.parent != *version || version.height.checked_add(1) != Some(journal.target_height) {
        return Err(StateError::ParentMismatch);
    }
    let required = estimate_journal_candidate_reservation(parent, journal, budget)?;
    if reserved_candidate_bytes < required {
        return Err(StateError::BudgetExceeded);
    }
    apply_state_journal(parent, version, journal, budget)
}
