// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{StateError, state::encoding::journal_decoding::take_borrowed_bytes};

pub(super) fn scan_commit_byte_list(
    mut list: &[u8],
    maximum_count: usize,
    maximum_item: usize,
    maximum_total: usize,
) -> Result<(usize, usize), StateError> {
    let mut count = 0_usize;
    let mut total = 0_usize;
    while !list.is_empty() {
        if count >= maximum_count {
            return Err(StateError::BudgetExceeded);
        }
        let bytes = take_borrowed_bytes(&mut list, maximum_item)?;
        total = total
            .checked_add(bytes.len())
            .ok_or(StateError::ArithmeticOverflow)?;
        if total > maximum_total {
            return Err(StateError::BudgetExceeded);
        }
        count = count.checked_add(1).ok_or(StateError::ArithmeticOverflow)?;
    }
    Ok((count, total))
}
