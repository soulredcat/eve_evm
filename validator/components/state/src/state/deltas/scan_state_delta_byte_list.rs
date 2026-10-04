// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::StateDeltaError;

pub(super) fn scan_state_delta_byte_list(
    mut bytes: &[u8],
    maximum_count: usize,
    maximum_item: usize,
    maximum_total: usize,
) -> Result<(usize, usize), StateDeltaError> {
    let mut count = 0_usize;
    let mut total = 0_usize;
    while !bytes.is_empty() {
        if count >= maximum_count {
            return Err(StateDeltaError::BudgetExceeded);
        }
        let field = alloy_rlp::Header::decode_bytes(&mut bytes, false)
            .map_err(|_| StateDeltaError::MalformedEncoding)?;
        total = total
            .checked_add(field.len())
            .ok_or(StateDeltaError::BudgetExceeded)?;
        if field.len() > maximum_item || total > maximum_total {
            return Err(StateDeltaError::BudgetExceeded);
        }
        count = count
            .checked_add(1)
            .ok_or(StateDeltaError::BudgetExceeded)?;
    }
    Ok((count, total))
}
