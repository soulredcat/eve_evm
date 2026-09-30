use super::estimate_operation_bytes::estimate_operation_bytes;
use crate::{StateBudget, StateError, StateJournal};

pub(crate) fn validate_journal_budget(
    journal: &StateJournal,
    budget: &StateBudget,
) -> Result<(), StateError> {
    if journal.operations.len() > budget.maximum_journal_operations
        || journal.parent.identity.network_name.len() > 64
    {
        return Err(StateError::BudgetExceeded);
    }
    let mut bytes = 1_024_usize;
    for operation in &journal.operations {
        bytes = bytes
            .checked_add(estimate_operation_bytes(operation, budget)?)
            .ok_or(StateError::ArithmeticOverflow)?;
        if bytes > budget.maximum_journal_bytes {
            return Err(StateError::BudgetExceeded);
        }
    }
    if bytes > budget.maximum_journal_bytes {
        return Err(StateError::BudgetExceeded);
    }
    Ok(())
}
