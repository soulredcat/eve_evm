// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_value, take_list};
use crate::{StateBudget, StateError};
use eve_protocol_config::records::ExecutionBlockHash;
use std::collections::BTreeMap;

pub(crate) fn decode_history(
    input: &mut &[u8],
    budget: &StateBudget,
) -> Result<BTreeMap<u64, ExecutionBlockHash>, StateError> {
    let mut list = take_list(input)?;
    let mut history = BTreeMap::new();
    while !list.is_empty() {
        if history.len() >= budget.maximum_block_hashes {
            return Err(StateError::BudgetExceeded);
        }
        let mut entry = take_list(&mut list)?;
        let height = decode_value(&mut entry)?;
        let hash = ExecutionBlockHash(decode_value(&mut entry)?);
        if !entry.is_empty() || history.insert(height, hash).is_some() {
            return Err(StateError::NonCanonicalEncoding);
        }
    }
    Ok(history)
}
