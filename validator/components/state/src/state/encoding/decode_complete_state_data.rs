// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_accounts::decode_accounts, decode_codes::decode_codes, decode_history::decode_history,
    decode_identity, decode_system_map::decode_system_map, take_list,
};
use crate::{CompleteState, StateBudget, StateError};

pub(crate) fn decode_complete_state_data(
    input: &mut &[u8],
    budget: &StateBudget,
) -> Result<CompleteState, StateError> {
    let before = *input;
    let mut list = take_list(input)?;
    if before.len() - input.len() > budget.maximum_state_bytes {
        return Err(StateError::BudgetExceeded);
    }
    let state = CompleteState {
        identity: decode_identity(&mut list)?,
        accounts: decode_accounts(&mut list, budget)?,
        codes: decode_codes(&mut list, budget)?,
        system: decode_system_map(&mut list, budget)?,
        block_hashes: decode_history(&mut list, budget)?,
    };
    if !list.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    Ok(state)
}
