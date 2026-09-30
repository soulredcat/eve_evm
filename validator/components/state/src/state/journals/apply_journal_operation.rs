use crate::{CompleteState, JournalOperation, StateAccount, StateBudget, StateError};
use alloy_primitives::keccak256;
use eve_protocol_config::records::{encode_system_record, hash_system_key};
use std::collections::BTreeMap;

pub(crate) fn apply_journal_operation(
    state: &mut CompleteState,
    operation: &JournalOperation,
    target_height: u64,
    budget: &StateBudget,
) -> Result<(), StateError> {
    match operation {
        JournalOperation::PutAccount {
            address,
            nonce,
            balance,
            code_hash,
        } => {
            let account = state.accounts.entry(*address).or_insert(StateAccount {
                nonce: *nonce,
                balance: *balance,
                code_hash: *code_hash,
                storage: BTreeMap::new(),
            });
            account.nonce = *nonce;
            account.balance = *balance;
            account.code_hash = *code_hash;
        }
        JournalOperation::DeleteAccount { address } => {
            state.accounts.remove(address);
        }
        JournalOperation::PutStorage {
            address,
            slot,
            value,
        } => {
            let account = state
                .accounts
                .get_mut(address)
                .ok_or(StateError::MissingAccount(*address))?;
            if value.is_zero() {
                account.storage.remove(slot);
            } else {
                account.storage.insert(*slot, *value);
            }
        }
        JournalOperation::DeleteStorage { address, slot } => {
            state
                .accounts
                .get_mut(address)
                .ok_or(StateError::MissingAccount(*address))?
                .storage
                .remove(slot);
        }
        JournalOperation::ClearStorage { address } => {
            state
                .accounts
                .get_mut(address)
                .ok_or(StateError::MissingAccount(*address))?
                .storage
                .clear();
        }
        JournalOperation::PutCode { code_hash, code } => {
            if code.len() > budget.maximum_code_bytes || keccak256(code) != *code_hash {
                return Err(StateError::CodeHashMismatch(*code_hash));
            }
            state.codes.insert(*code_hash, code.clone());
        }
        JournalOperation::DeleteCode { code_hash } => {
            state.codes.remove(code_hash);
        }
        JournalOperation::PutSystem { key, record } => {
            encode_system_record(record).map_err(StateError::SystemRecord)?;
            if hash_system_key(record.namespace, &record.logical_key)
                .map_err(StateError::SystemRecord)?
                != *key
            {
                return Err(StateError::SystemKeyMismatch);
            }
            state.system.insert(*key, record.clone());
        }
        JournalOperation::DeleteSystem { key } => {
            state.system.remove(key);
        }
        JournalOperation::SetExecutionBlockHash { height, hash } => {
            if *height != target_height || state.block_hashes.contains_key(height) {
                return Err(StateError::VersionMismatch);
            }
            state.block_hashes.insert(*height, *hash);
        }
    }
    Ok(())
}
