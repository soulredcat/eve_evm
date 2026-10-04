// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateCommitDecodeStats;
use crate::{
    B256, StateBudget, StateError,
    state::encoding::{decode_value, take_list},
};

pub(super) fn scan_commit_history(
    input: &mut &[u8],
    budget: &StateBudget,
    stats: &mut StateCommitDecodeStats,
) -> Result<(), StateError> {
    let mut history = take_list(input)?;
    let mut previous = None;
    while !history.is_empty() {
        if stats.history_entries >= budget.maximum_block_hashes {
            return Err(StateError::BudgetExceeded);
        }
        let mut entry = take_list(&mut history)?;
        let height = decode_value::<u64>(&mut entry)?;
        decode_value::<B256>(&mut entry)?;
        if !entry.is_empty() || previous.is_some_and(|old| old >= height) {
            return Err(StateError::NonCanonicalEncoding);
        }
        previous = Some(height);
        stats.history_entries = stats
            .history_entries
            .checked_add(1)
            .ok_or(StateError::ArithmeticOverflow)?;
    }
    Ok(())
}
