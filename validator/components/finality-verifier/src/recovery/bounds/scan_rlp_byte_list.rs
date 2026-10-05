// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::{
    RecoveryError, decoding::take_bounded_rlp_payload::take_bounded_rlp_payload,
};

pub(crate) fn scan_rlp_byte_list(
    mut input: &[u8],
    maximum_count: usize,
    maximum_item: usize,
    maximum_total: usize,
) -> Result<(usize, usize), RecoveryError> {
    let mut count = 0_usize;
    let mut total = 0_usize;
    while !input.is_empty() {
        if count >= maximum_count {
            return Err(RecoveryError::BudgetExceeded);
        }
        let payload = take_bounded_rlp_payload(&mut input, false, maximum_item)?;
        total = total
            .checked_add(payload.len())
            .ok_or(RecoveryError::BudgetExceeded)?;
        if total > maximum_total {
            return Err(RecoveryError::BudgetExceeded);
        }
        count += 1;
    }
    Ok((count, total))
}
