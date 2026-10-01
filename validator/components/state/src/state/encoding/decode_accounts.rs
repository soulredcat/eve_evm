// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_value, take_list};
use crate::{StateAccount, StateBudget, StateError};
use alloy_primitives::{Address, U256};
use std::collections::BTreeMap;

pub(crate) fn decode_accounts(
    input: &mut &[u8],
    budget: &StateBudget,
) -> Result<BTreeMap<Address, StateAccount>, StateError> {
    let mut list = take_list(input)?;
    let mut accounts = BTreeMap::new();
    let mut total_slots = 0_usize;
    while !list.is_empty() {
        if accounts.len() >= budget.maximum_accounts {
            return Err(StateError::BudgetExceeded);
        }
        let mut entry = take_list(&mut list)?;
        let address = decode_value(&mut entry)?;
        let nonce = decode_value(&mut entry)?;
        let balance = decode_value(&mut entry)?;
        let code_hash = decode_value(&mut entry)?;
        let mut slots = take_list(&mut entry)?;
        let mut storage = BTreeMap::new();
        while !slots.is_empty() {
            total_slots = total_slots
                .checked_add(1)
                .ok_or(StateError::ArithmeticOverflow)?;
            if total_slots > budget.maximum_storage_slots {
                return Err(StateError::BudgetExceeded);
            }
            let mut pair = take_list(&mut slots)?;
            let key: U256 = decode_value(&mut pair)?;
            let value = decode_value(&mut pair)?;
            if !pair.is_empty() || storage.insert(key, value).is_some() {
                return Err(StateError::NonCanonicalEncoding);
            }
        }
        if !entry.is_empty()
            || accounts
                .insert(
                    address,
                    StateAccount {
                        nonce,
                        balance,
                        code_hash,
                        storage,
                    },
                )
                .is_some()
        {
            return Err(StateError::NonCanonicalEncoding);
        }
    }
    Ok(accounts)
}
