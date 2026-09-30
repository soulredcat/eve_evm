use super::{apply_journal_operation::apply_journal_operation, encode_state_journal};
use crate::{
    CompleteState, StateBudget, StateError, StateJournal, StateVersion, validate_complete_state,
    validate_state_version,
};

/// Apply to a private candidate; any error leaves the parent untouched.
pub fn apply_state_journal(
    parent: &CompleteState,
    version: &StateVersion,
    journal: &StateJournal,
    budget: &StateBudget,
) -> Result<CompleteState, StateError> {
    validate_state_version(parent, version, budget)?;
    if journal.parent != *version || version.height.checked_add(1) != Some(journal.target_height) {
        return Err(StateError::ParentMismatch);
    }
    if journal.operations.len() > budget.maximum_journal_operations
        || encode_state_journal(journal, budget)?.len() > budget.maximum_journal_bytes
    {
        return Err(StateError::BudgetExceeded);
    }
    let mut candidate = parent.clone();
    for operation in &journal.operations {
        apply_journal_operation(&mut candidate, operation, journal.target_height, budget)?;
    }
    validate_complete_state(&candidate, budget)?;
    Ok(candidate)
}
