use super::push_journal_operation::push_journal_operation;
use crate::{
    CompleteState, JournalOperation, StateBudget, StateError, StateJournal, StateVersion,
    validate_complete_state, validate_state_version,
};
use std::collections::BTreeSet;

/// Deterministic complete-state differences with bounded incremental allocation.
pub fn project_state_journal(
    parent: &CompleteState,
    version: &StateVersion,
    candidate: &CompleteState,
    target_height: u64,
    budget: &StateBudget,
) -> Result<StateJournal, StateError> {
    validate_state_version(parent, version, budget)?;
    validate_complete_state(candidate, budget)?;
    if parent.identity != candidate.identity || version.height.checked_add(1) != Some(target_height)
    {
        return Err(StateError::ParentMismatch);
    }
    let mut operations = Vec::new();
    let mut bytes = 1_024_usize;
    let mut push =
        |operation| push_journal_operation(&mut operations, &mut bytes, operation, budget);
    for (hash, code) in &candidate.codes {
        if parent.codes.get(hash) != Some(code) {
            push(JournalOperation::PutCode {
                code_hash: *hash,
                code: code.clone(),
            })?;
        }
    }
    let addresses = parent
        .accounts
        .keys()
        .chain(candidate.accounts.keys())
        .copied()
        .collect::<BTreeSet<_>>();
    for address in addresses {
        let old = parent.accounts.get(&address);
        let Some(new) = candidate.accounts.get(&address) else {
            push(JournalOperation::DeleteAccount { address })?;
            continue;
        };
        if old.is_none_or(|old| {
            old.nonce != new.nonce || old.balance != new.balance || old.code_hash != new.code_hash
        }) {
            push(JournalOperation::PutAccount {
                address,
                nonce: new.nonce,
                balance: new.balance,
                code_hash: new.code_hash,
            })?;
        }
        let slots = old
            .into_iter()
            .flat_map(|old| old.storage.keys())
            .chain(new.storage.keys())
            .copied()
            .collect::<BTreeSet<_>>();
        for slot in slots {
            let prior = old.and_then(|old| old.storage.get(&slot));
            match new.storage.get(&slot) {
                Some(value) if Some(value) != prior => push(JournalOperation::PutStorage {
                    address,
                    slot,
                    value: *value,
                })?,
                None if prior.is_some() => push(JournalOperation::DeleteStorage { address, slot })?,
                _ => {}
            }
        }
    }
    for hash in parent
        .codes
        .keys()
        .filter(|hash| !candidate.codes.contains_key(*hash))
    {
        push(JournalOperation::DeleteCode { code_hash: *hash })?;
    }
    for (key, record) in &candidate.system {
        if parent.system.get(key) != Some(record) {
            push(JournalOperation::PutSystem {
                key: *key,
                record: record.clone(),
            })?;
        }
    }
    for key in parent
        .system
        .keys()
        .filter(|key| !candidate.system.contains_key(*key))
    {
        push(JournalOperation::DeleteSystem { key: *key })?;
    }
    for (height, hash) in &candidate.block_hashes {
        if parent.block_hashes.get(height) != Some(hash) {
            if *height != target_height || parent.block_hashes.contains_key(height) {
                return Err(StateError::VersionMismatch);
            }
            push(JournalOperation::SetExecutionBlockHash {
                height: *height,
                hash: *hash,
            })?;
        }
    }
    if parent
        .block_hashes
        .keys()
        .any(|height| !candidate.block_hashes.contains_key(height))
    {
        return Err(StateError::VersionMismatch);
    }
    let journal = StateJournal {
        parent: version.clone(),
        target_height,
        operations,
    };
    super::validate_journal_budget::validate_journal_budget(&journal, budget)?;
    Ok(journal)
}
