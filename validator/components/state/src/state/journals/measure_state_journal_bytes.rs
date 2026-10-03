// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    operation_journal_wire_length::operation_journal_wire_length,
    validate_journal_budget::validate_journal_budget,
};
use crate::{
    StateBudget, StateError, StateJournal,
    state::{encoding::encode_version, limits::list_length},
};
use alloy_rlp::Encodable;

/// Exact existing canonical journal wire length without whole-journal encoding.
/// Version/system codecs use bounded field scratch (each at most 4096 bytes);
/// this is not allocation-free overall. Existing conservative admission remains.
pub fn measure_state_journal_bytes(
    journal: &StateJournal,
    budget: &StateBudget,
) -> Result<usize, StateError> {
    validate_journal_budget(journal, budget)?;
    let mut operation_bytes = 0_usize;
    for operation in &journal.operations {
        operation_bytes = operation_bytes
            .checked_add(operation_journal_wire_length(operation)?)
            .ok_or(StateError::ArithmeticOverflow)?;
    }
    let payload = [
        b"EVE_STATE_JOURNAL_V1".as_slice().length(),
        encode_version(&journal.parent).len(),
        journal.target_height.length(),
        list_length(operation_bytes)?,
    ]
    .into_iter()
    .try_fold(0_usize, |sum, field| sum.checked_add(field))
    .ok_or(StateError::ArithmeticOverflow)?;
    let length = list_length(payload)?;
    if length > budget.maximum_journal_bytes {
        return Err(StateError::BudgetExceeded);
    }
    Ok(length)
}
