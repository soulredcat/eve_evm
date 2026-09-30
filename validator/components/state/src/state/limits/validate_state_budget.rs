use crate::{StateBudget, StateError};

pub(crate) fn validate_state_budget(budget: &StateBudget) -> Result<(), StateError> {
    if [
        budget.maximum_accounts,
        budget.maximum_storage_slots,
        budget.maximum_codes,
        budget.maximum_code_bytes,
        budget.maximum_total_code_bytes,
        budget.maximum_system_records,
        budget.maximum_system_bytes,
        budget.maximum_block_hashes,
        budget.maximum_journal_operations,
        budget.maximum_journal_bytes,
        budget.maximum_state_bytes,
        budget.maximum_commit_bytes,
    ]
    .contains(&0)
        || budget.maximum_code_bytes > 24_576
        || budget.maximum_code_bytes > budget.maximum_total_code_bytes
        || budget.maximum_total_code_bytes > budget.maximum_state_bytes
        || budget.maximum_system_bytes > budget.maximum_state_bytes
        || budget.maximum_state_bytes > budget.maximum_commit_bytes
    {
        return Err(StateError::BudgetExceeded);
    }
    Ok(())
}
