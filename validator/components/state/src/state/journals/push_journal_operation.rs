// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::estimate_operation_bytes::estimate_operation_bytes;
use crate::{JournalOperation, StateBudget, StateError};

pub(crate) fn push_journal_operation(
    operations: &mut Vec<JournalOperation>,
    bytes: &mut usize,
    operation: JournalOperation,
    budget: &StateBudget,
) -> Result<(), StateError> {
    if operations.len() >= budget.maximum_journal_operations {
        return Err(StateError::BudgetExceeded);
    }
    let next = bytes
        .checked_add(estimate_operation_bytes(&operation, budget)?)
        .ok_or(StateError::ArithmeticOverflow)?;
    if next > budget.maximum_journal_bytes {
        return Err(StateError::BudgetExceeded);
    }
    operations.push(operation);
    *bytes = next;
    Ok(())
}
