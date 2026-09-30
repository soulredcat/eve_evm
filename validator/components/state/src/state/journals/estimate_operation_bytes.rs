use crate::{JournalOperation, StateBudget, StateError};
use eve_protocol_config::records::encode_system_record;

pub(crate) fn estimate_operation_bytes(
    operation: &JournalOperation,
    budget: &StateBudget,
) -> Result<usize, StateError> {
    match operation {
        JournalOperation::PutCode { code, .. } => {
            if code.len() > budget.maximum_code_bytes {
                return Err(StateError::BudgetExceeded);
            }
            code.len()
                .checked_add(64)
                .ok_or(StateError::ArithmeticOverflow)
        }
        JournalOperation::PutSystem { record, .. } => encode_system_record(record)
            .map_err(StateError::SystemRecord)?
            .len()
            .checked_add(64)
            .ok_or(StateError::ArithmeticOverflow),
        JournalOperation::PutAccount { .. } | JournalOperation::PutStorage { .. } => Ok(128),
        _ => Ok(64),
    }
}
