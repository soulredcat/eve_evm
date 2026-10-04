// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateCommitDecodeStats;
use crate::{
    Address, B256, StateBudget, StateError, U256,
    state::encoding::{decode_value, take_list},
};

pub(super) fn scan_commit_accounts(
    input: &mut &[u8],
    budget: &StateBudget,
    stats: &mut StateCommitDecodeStats,
) -> Result<(), StateError> {
    let mut accounts = take_list(input)?;
    let mut previous = None;
    while !accounts.is_empty() {
        if stats.accounts >= budget.maximum_accounts {
            return Err(StateError::BudgetExceeded);
        }
        let mut entry = take_list(&mut accounts)?;
        let address = decode_value::<Address>(&mut entry)?;
        if previous.is_some_and(|old| old >= address) {
            return Err(StateError::NonCanonicalEncoding);
        }
        previous = Some(address);
        decode_value::<u64>(&mut entry)?;
        decode_value::<U256>(&mut entry)?;
        decode_value::<B256>(&mut entry)?;
        let mut storage = take_list(&mut entry)?;
        let mut previous_slot = None;
        while !storage.is_empty() {
            if stats.storage_slots >= budget.maximum_storage_slots {
                return Err(StateError::BudgetExceeded);
            }
            let mut pair = take_list(&mut storage)?;
            let slot = decode_value::<U256>(&mut pair)?;
            decode_value::<U256>(&mut pair)?;
            if !pair.is_empty() || previous_slot.is_some_and(|old| old >= slot) {
                return Err(StateError::NonCanonicalEncoding);
            }
            previous_slot = Some(slot);
            stats.storage_slots = stats
                .storage_slots
                .checked_add(1)
                .ok_or(StateError::ArithmeticOverflow)?;
        }
        if !entry.is_empty() {
            return Err(StateError::MalformedEncoding);
        }
        stats.accounts = stats
            .accounts
            .checked_add(1)
            .ok_or(StateError::ArithmeticOverflow)?;
    }
    Ok(())
}
