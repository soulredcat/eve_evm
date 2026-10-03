// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    BorrowedJournalOperation, JournalDecodePreflight, decode_borrowed_operation,
    scan_record_allocations::scan_record_allocations,
    scan_version_allocations::scan_version_allocations, take_borrowed_bytes, take_encoded_list,
};
use crate::{
    JournalOperation, StateBudget, StateError,
    state::encoding::{decode_value, take_list},
};

/// Allocation-free wire/budget preflight. It authenticates neither parent nor outcome.
/// Record semantic/key validation remains in the maintained decoder and apply path.
/// Aggregate payload admission counts every occurrence, including repeated writes;
/// these operator decode limits do not define final-state cardinality or consensus.
pub fn preflight_state_journal(
    bytes: &[u8],
    budget: &StateBudget,
) -> Result<JournalDecodePreflight, StateError> {
    if bytes.len() > budget.maximum_journal_bytes {
        return Err(StateError::BudgetExceeded);
    }
    let mut remaining = bytes;
    let mut fields = take_list(&mut remaining)?;
    if take_borrowed_bytes(&mut fields, 32)? != b"EVE_STATE_JOURNAL_V1" {
        return Err(StateError::InvalidSchema);
    }
    let parent = take_encoded_list(&mut fields, 4_096)?;
    let parent_network_name_bytes = scan_version_allocations(parent)?;
    decode_value::<u64>(&mut fields)?;
    let mut operations = take_list(&mut fields)?;
    if !fields.is_empty() || !remaining.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    let mut counts = JournalDecodePreflight {
        encoded_bytes: bytes.len(),
        parent_network_name_bytes,
        conservative_journal_bytes: 1_024,
        ..JournalDecodePreflight::default()
    };
    while !operations.is_empty() {
        if counts.operation_count >= budget.maximum_journal_operations {
            return Err(StateError::BudgetExceeded);
        }
        let operation = decode_borrowed_operation(&mut operations, budget)?;
        let operation_bytes = match operation {
            BorrowedJournalOperation::PutCode { code, .. } => {
                counts.code_operations = counts
                    .code_operations
                    .checked_add(1)
                    .ok_or(StateError::ArithmeticOverflow)?;
                counts.code_bytes = counts
                    .code_bytes
                    .checked_add(code.len())
                    .ok_or(StateError::ArithmeticOverflow)?;
                if counts.code_bytes > budget.maximum_total_code_bytes {
                    return Err(StateError::BudgetExceeded);
                }
                code.len()
                    .checked_add(64)
                    .ok_or(StateError::ArithmeticOverflow)?
            }
            BorrowedJournalOperation::PutSystem { record, .. } => {
                let (payload_bytes, leaves) = scan_record_allocations(record)?;
                counts.system_operations = counts
                    .system_operations
                    .checked_add(1)
                    .ok_or(StateError::ArithmeticOverflow)?;
                counts.system_encoded_bytes = counts
                    .system_encoded_bytes
                    .checked_add(record.len())
                    .ok_or(StateError::ArithmeticOverflow)?;
                counts.system_payload_bytes = counts
                    .system_payload_bytes
                    .checked_add(payload_bytes)
                    .ok_or(StateError::ArithmeticOverflow)?;
                counts.system_leaf_count = counts
                    .system_leaf_count
                    .checked_add(leaves)
                    .ok_or(StateError::ArithmeticOverflow)?;
                counts.maximum_system_record_bytes =
                    counts.maximum_system_record_bytes.max(record.len());
                if counts.system_encoded_bytes > budget.maximum_system_bytes {
                    return Err(StateError::BudgetExceeded);
                }
                record
                    .len()
                    .checked_add(64)
                    .ok_or(StateError::ArithmeticOverflow)?
            }
            BorrowedJournalOperation::PutAccount { .. } => {
                counts.account_operations = counts
                    .account_operations
                    .checked_add(1)
                    .ok_or(StateError::ArithmeticOverflow)?;
                128
            }
            BorrowedJournalOperation::PutStorage { .. } => {
                counts.storage_operations = counts
                    .storage_operations
                    .checked_add(1)
                    .ok_or(StateError::ArithmeticOverflow)?;
                128
            }
            BorrowedJournalOperation::SetExecutionBlockHash { .. } => {
                counts.execution_hash_operations = counts
                    .execution_hash_operations
                    .checked_add(1)
                    .ok_or(StateError::ArithmeticOverflow)?;
                64
            }
            _ => 64,
        };
        counts.operation_count = counts
            .operation_count
            .checked_add(1)
            .ok_or(StateError::ArithmeticOverflow)?;
        counts.conservative_journal_bytes = counts
            .conservative_journal_bytes
            .checked_add(operation_bytes)
            .ok_or(StateError::ArithmeticOverflow)?;
        if counts.conservative_journal_bytes > budget.maximum_journal_bytes {
            return Err(StateError::BudgetExceeded);
        }
    }
    if counts.conservative_journal_bytes > budget.maximum_journal_bytes {
        return Err(StateError::BudgetExceeded);
    }
    counts.operation_allocation_bytes = counts
        .operation_count
        .checked_mul(core::mem::size_of::<JournalOperation>())
        .ok_or(StateError::ArithmeticOverflow)?;
    // Sequential codecs keep only one record's temporary encodings at a time.
    // This policy estimate includes primitive-field Vec scaffolding and copies;
    // allocator metadata and the caller's input/retained state are separate charges.
    counts.conservative_codec_scratch_bytes = counts
        .maximum_system_record_bytes
        .checked_mul(8)
        .and_then(|value| value.checked_add(16_384))
        .ok_or(StateError::ArithmeticOverflow)?;
    Ok(counts)
}
