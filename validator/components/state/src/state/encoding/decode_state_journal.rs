// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_value,
    decode_version::decode_version,
    journal_decoding::{
        decode_borrowed_operation, materialize_operation, preflight_state_journal,
        take_borrowed_bytes,
    },
    take_list,
};
use crate::{StateBudget, StateError, StateJournal};

/// Decode canonical ordered operations only after the complete borrowed preflight.
/// No execution, finality authentication, or final-state validation is implied.
pub fn decode_state_journal(
    bytes: &[u8],
    budget: &StateBudget,
) -> Result<StateJournal, StateError> {
    let preflight = preflight_state_journal(bytes, budget)?;
    let mut remaining = bytes;
    let mut fields = take_list(&mut remaining)?;
    take_borrowed_bytes(&mut fields, 32)?;
    let parent = decode_version(&mut fields)?;
    let target_height = decode_value(&mut fields)?;
    let mut encoded_operations = take_list(&mut fields)?;
    let mut operations = Vec::new();
    operations
        .try_reserve_exact(preflight.operation_count)
        .map_err(|_| StateError::BudgetExceeded)?;
    while !encoded_operations.is_empty() {
        let operation = decode_borrowed_operation(&mut encoded_operations, budget)?;
        operations.push(materialize_operation(operation)?);
    }
    Ok(StateJournal {
        parent,
        target_height,
        operations,
    })
}
