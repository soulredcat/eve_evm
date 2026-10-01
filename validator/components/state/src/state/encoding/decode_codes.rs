// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_value, take_bytes, take_list};
use crate::{StateBudget, StateError};
use alloy_primitives::{B256, Bytes};
use std::collections::BTreeMap;

pub(crate) fn decode_codes(
    input: &mut &[u8],
    budget: &StateBudget,
) -> Result<BTreeMap<B256, Bytes>, StateError> {
    let mut list = take_list(input)?;
    let mut codes = BTreeMap::new();
    let mut total = 0_usize;
    while !list.is_empty() {
        if codes.len() >= budget.maximum_codes {
            return Err(StateError::BudgetExceeded);
        }
        let mut entry = take_list(&mut list)?;
        let key = decode_value(&mut entry)?;
        let code = take_bytes(&mut entry, budget.maximum_code_bytes)?;
        total = total
            .checked_add(code.len())
            .ok_or(StateError::ArithmeticOverflow)?;
        if total > budget.maximum_total_code_bytes {
            return Err(StateError::BudgetExceeded);
        }
        if !entry.is_empty() || codes.insert(key, code).is_some() {
            return Err(StateError::NonCanonicalEncoding);
        }
    }
    Ok(codes)
}
