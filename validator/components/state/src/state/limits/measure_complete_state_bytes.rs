use super::list_length::list_length;
use crate::state::encoding::encode_identity;
use crate::{CompleteState, StateBudget, StateError};
use alloy_rlp::Encodable;
use eve_protocol_config::records::encode_system_record;

/// Exact canonical byte length without allocating account/slot/code serialization.
/// System record encoding is individually bounded by the canonical schema.
pub fn measure_complete_state_bytes(
    state: &CompleteState,
    budget: &StateBudget,
) -> Result<usize, StateError> {
    let cap = budget.maximum_state_bytes;
    let identity = encode_identity(&state.identity).len();
    if identity > cap {
        return Err(StateError::BudgetExceeded);
    }
    let mut accounts = 0_usize;
    for (address, account) in &state.accounts {
        let mut slots = 0_usize;
        for (key, value) in &account.storage {
            let pair = key
                .length()
                .checked_add(value.length())
                .ok_or(StateError::ArithmeticOverflow)?;
            slots = slots
                .checked_add(list_length(pair)?)
                .ok_or(StateError::ArithmeticOverflow)?;
            if slots > cap {
                return Err(StateError::BudgetExceeded);
            }
        }
        let entry = [
            address.length(),
            account.nonce.length(),
            account.balance.length(),
            account.code_hash.length(),
            list_length(slots)?,
        ]
        .into_iter()
        .try_fold(0_usize, |sum, value| sum.checked_add(value))
        .ok_or(StateError::ArithmeticOverflow)?;
        accounts = accounts
            .checked_add(list_length(entry)?)
            .ok_or(StateError::ArithmeticOverflow)?;
        if accounts > cap {
            return Err(StateError::BudgetExceeded);
        }
    }
    let mut codes = 0_usize;
    for (hash, code) in &state.codes {
        let pair = hash
            .length()
            .checked_add(code.as_ref().length())
            .ok_or(StateError::ArithmeticOverflow)?;
        codes = codes
            .checked_add(list_length(pair)?)
            .ok_or(StateError::ArithmeticOverflow)?;
        if codes > cap {
            return Err(StateError::BudgetExceeded);
        }
    }
    let mut system = 0_usize;
    for (key, record) in &state.system {
        let pair = key
            .length()
            .checked_add(
                encode_system_record(record)
                    .map_err(StateError::SystemRecord)?
                    .len(),
            )
            .ok_or(StateError::ArithmeticOverflow)?;
        system = system
            .checked_add(list_length(pair)?)
            .ok_or(StateError::ArithmeticOverflow)?;
        if system > cap {
            return Err(StateError::BudgetExceeded);
        }
    }
    let mut history = 0_usize;
    for (height, hash) in &state.block_hashes {
        let pair = height
            .length()
            .checked_add(hash.0.length())
            .ok_or(StateError::ArithmeticOverflow)?;
        history = history
            .checked_add(list_length(pair)?)
            .ok_or(StateError::ArithmeticOverflow)?;
        if history > cap {
            return Err(StateError::BudgetExceeded);
        }
    }
    let payload = [
        identity,
        list_length(accounts)?,
        list_length(codes)?,
        list_length(system)?,
        list_length(history)?,
    ]
    .into_iter()
    .try_fold(0_usize, |sum, value| sum.checked_add(value))
    .ok_or(StateError::ArithmeticOverflow)?;
    let length = list_length(payload)?;
    if length > cap {
        return Err(StateError::BudgetExceeded);
    }
    Ok(length)
}
