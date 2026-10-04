// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateCommitDecodeStats;
use crate::{
    B256, StateBudget, StateError,
    state::encoding::{
        decode_value,
        journal_decoding::{scan_record_allocations, take_encoded_list},
        take_list,
    },
};

pub(super) fn scan_commit_system(
    input: &mut &[u8],
    budget: &StateBudget,
    stats: &mut StateCommitDecodeStats,
) -> Result<(), StateError> {
    let mut records = take_list(input)?;
    let mut previous = None;
    while !records.is_empty() {
        if stats.system_records >= budget.maximum_system_records {
            return Err(StateError::BudgetExceeded);
        }
        let mut entry = take_list(&mut records)?;
        let key = decode_value::<B256>(&mut entry)?;
        if previous.is_some_and(|old| old >= key) {
            return Err(StateError::NonCanonicalEncoding);
        }
        previous = Some(key);
        let record = take_encoded_list(&mut entry, 4_096.min(budget.maximum_system_bytes))?;
        let (payload, leaves) = scan_record_allocations(record)?;
        stats.system_encoded_bytes = stats
            .system_encoded_bytes
            .checked_add(record.len())
            .ok_or(StateError::ArithmeticOverflow)?;
        stats.system_payload_bytes = stats
            .system_payload_bytes
            .checked_add(payload)
            .ok_or(StateError::ArithmeticOverflow)?;
        stats.system_leaf_count = stats
            .system_leaf_count
            .checked_add(leaves)
            .ok_or(StateError::ArithmeticOverflow)?;
        stats.maximum_system_record_bytes = stats.maximum_system_record_bytes.max(record.len());
        if stats.system_encoded_bytes > budget.maximum_system_bytes {
            return Err(StateError::BudgetExceeded);
        }
        if !entry.is_empty() {
            return Err(StateError::MalformedEncoding);
        }
        stats.system_records = stats
            .system_records
            .checked_add(1)
            .ok_or(StateError::ArithmeticOverflow)?;
    }
    Ok(())
}
