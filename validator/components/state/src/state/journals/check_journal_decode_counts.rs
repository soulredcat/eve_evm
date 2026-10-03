// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{JournalDecodePreflight, JournalOperation, StateBudget, StateError};

/// Consistency/admission only. A caller must scan its actual immutable wire bytes.
pub(crate) fn check_journal_decode_counts(
    counts: &JournalDecodePreflight,
    budget: &StateBudget,
) -> Result<(), StateError> {
    let wide_operations = counts
        .account_operations
        .checked_add(counts.storage_operations)
        .ok_or(StateError::ArithmeticOverflow)?;
    let insertions = [
        counts.code_operations,
        counts.system_operations,
        counts.execution_hash_operations,
    ]
    .into_iter()
    .try_fold(wide_operations, |sum, count| sum.checked_add(count))
    .ok_or(StateError::ArithmeticOverflow)?;
    if insertions > counts.operation_count {
        return Err(StateError::MalformedEncoding);
    }
    let ordinary_operations = counts
        .operation_count
        .checked_sub(wide_operations)
        .ok_or(StateError::ArithmeticOverflow)?;
    let journal_bytes = wide_operations
        .checked_mul(128)
        .and_then(|wide| ordinary_operations.checked_mul(64)?.checked_add(wide))
        .and_then(|bytes| bytes.checked_add(counts.code_bytes))
        .and_then(|bytes| bytes.checked_add(counts.system_encoded_bytes))
        .and_then(|bytes| bytes.checked_add(1_024))
        .ok_or(StateError::ArithmeticOverflow)?;
    let operation_bytes = counts
        .operation_count
        .checked_mul(core::mem::size_of::<JournalOperation>())
        .ok_or(StateError::ArithmeticOverflow)?;
    if journal_bytes != counts.conservative_journal_bytes
        || operation_bytes != counts.operation_allocation_bytes
        || counts.parent_network_name_bytes > 64
        || counts.maximum_system_record_bytes > 4_096
        || counts.maximum_system_record_bytes > counts.system_encoded_bytes
        || (counts.code_operations == 0 && counts.code_bytes != 0)
        || (counts.system_operations == 0 && counts.system_encoded_bytes != 0)
    {
        return Err(StateError::MalformedEncoding);
    }
    if counts.operation_count > budget.maximum_journal_operations
        || counts.encoded_bytes > budget.maximum_journal_bytes
        || counts.conservative_journal_bytes > budget.maximum_journal_bytes
        || counts.code_bytes > budget.maximum_total_code_bytes
        || counts.system_encoded_bytes > budget.maximum_system_bytes
    {
        return Err(StateError::BudgetExceeded);
    }
    Ok(())
}
