use alloy_primitives::{U256, keccak256};
use alloy_trie::KECCAK_EMPTY;
use eve_protocol_config::records::{encode_system_record, hash_system_key};

use super::validate_state_identity;
use crate::state::limits::{measure_complete_state_bytes, validate_state_budget};
use crate::{CompleteState, StateBudget, StateError};

pub fn validate_complete_state(
    state: &CompleteState,
    budget: &StateBudget,
) -> Result<(), StateError> {
    validate_state_budget(budget)?;
    validate_state_identity(&state.identity)?;
    if state.accounts.len() > budget.maximum_accounts
        || state.codes.len() > budget.maximum_codes
        || state.system.len() > budget.maximum_system_records
        || state.block_hashes.len() > budget.maximum_block_hashes
    {
        return Err(StateError::BudgetExceeded);
    }
    let mut slots = 0_usize;
    for account in state.accounts.values() {
        slots = slots
            .checked_add(account.storage.len())
            .ok_or(StateError::ArithmeticOverflow)?;
        if slots > budget.maximum_storage_slots {
            return Err(StateError::BudgetExceeded);
        }
        if account.storage.values().any(U256::is_zero) {
            return Err(StateError::NonCanonicalStorage);
        }
        if account.code_hash != KECCAK_EMPTY && !state.codes.contains_key(&account.code_hash) {
            return Err(StateError::MissingCode(account.code_hash));
        }
    }
    if slots > budget.maximum_storage_slots {
        return Err(StateError::BudgetExceeded);
    }
    let mut code_bytes = 0_usize;
    for (hash, code) in &state.codes {
        code_bytes = code_bytes
            .checked_add(code.len())
            .ok_or(StateError::ArithmeticOverflow)?;
        if code_bytes > budget.maximum_total_code_bytes {
            return Err(StateError::BudgetExceeded);
        }
        if code.len() > budget.maximum_code_bytes || keccak256(code) != *hash {
            return Err(StateError::CodeHashMismatch(*hash));
        }
    }
    let mut system_bytes = 0_usize;
    for (key, record) in &state.system {
        if hash_system_key(record.namespace, &record.logical_key)
            .map_err(StateError::SystemRecord)?
            != *key
        {
            return Err(StateError::SystemKeyMismatch);
        }
        system_bytes = system_bytes
            .checked_add(
                encode_system_record(record)
                    .map_err(StateError::SystemRecord)?
                    .len(),
            )
            .ok_or(StateError::ArithmeticOverflow)?;
        if system_bytes > budget.maximum_system_bytes {
            return Err(StateError::BudgetExceeded);
        }
    }
    if code_bytes > budget.maximum_total_code_bytes
        || system_bytes > budget.maximum_system_bytes
        || measure_complete_state_bytes(state, budget)? > budget.maximum_state_bytes
    {
        return Err(StateError::BudgetExceeded);
    }
    Ok(())
}
