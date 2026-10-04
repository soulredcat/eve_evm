// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateCommitDecodeStats;
use crate::{
    B256, StateBudget, StateError,
    state::encoding::{decode_value, journal_decoding::take_borrowed_bytes, take_list},
};

pub(super) fn scan_commit_codes(
    input: &mut &[u8],
    budget: &StateBudget,
    stats: &mut StateCommitDecodeStats,
) -> Result<(), StateError> {
    let mut codes = take_list(input)?;
    let mut previous = None;
    while !codes.is_empty() {
        if stats.codes >= budget.maximum_codes {
            return Err(StateError::BudgetExceeded);
        }
        let mut entry = take_list(&mut codes)?;
        let hash = decode_value::<B256>(&mut entry)?;
        if previous.is_some_and(|old| old >= hash) {
            return Err(StateError::NonCanonicalEncoding);
        }
        previous = Some(hash);
        let code = take_borrowed_bytes(&mut entry, budget.maximum_code_bytes)?;
        stats.code_bytes = stats
            .code_bytes
            .checked_add(code.len())
            .ok_or(StateError::ArithmeticOverflow)?;
        stats.maximum_code_blob_bytes = stats.maximum_code_blob_bytes.max(code.len());
        if stats.code_bytes > budget.maximum_total_code_bytes {
            return Err(StateError::BudgetExceeded);
        }
        if !entry.is_empty() {
            return Err(StateError::MalformedEncoding);
        }
        stats.codes = stats
            .codes
            .checked_add(1)
            .ok_or(StateError::ArithmeticOverflow)?;
    }
    Ok(())
}
