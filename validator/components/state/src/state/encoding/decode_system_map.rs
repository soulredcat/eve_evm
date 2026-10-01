// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_system_record, decode_value, take_list};
use crate::{StateBudget, StateError, SystemRecord};
use alloy_primitives::B256;
use std::collections::BTreeMap;

pub(crate) fn decode_system_map(
    input: &mut &[u8],
    budget: &StateBudget,
) -> Result<BTreeMap<B256, SystemRecord>, StateError> {
    let mut list = take_list(input)?;
    let mut records = BTreeMap::new();
    let mut total = 0_usize;
    while !list.is_empty() {
        if records.len() >= budget.maximum_system_records {
            return Err(StateError::BudgetExceeded);
        }
        let mut entry = take_list(&mut list)?;
        let key = decode_value(&mut entry)?;
        let before = entry;
        take_list(&mut entry)?;
        let length = before.len() - entry.len();
        total = total
            .checked_add(length)
            .ok_or(StateError::ArithmeticOverflow)?;
        if total > budget.maximum_system_bytes {
            return Err(StateError::BudgetExceeded);
        }
        let record = decode_system_record(&before[..length])?;
        if !entry.is_empty() || records.insert(key, record).is_some() {
            return Err(StateError::NonCanonicalEncoding);
        }
    }
    Ok(records)
}
