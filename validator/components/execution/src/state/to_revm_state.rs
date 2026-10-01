// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CompleteExecutionError, estimate_clone_reservation};
use alloy_primitives::U256;
use eve_state::{CompleteState, StateBudget, StateError, StateVersion, validate_state_version};
use revm::{
    database::InMemoryDB,
    state::{AccountInfo, Bytecode},
};

pub fn to_revm_state(
    parent: &CompleteState,
    version: &StateVersion,
    budget: &StateBudget,
    reserved_clone_bytes: usize,
) -> Result<InMemoryDB, CompleteExecutionError> {
    validate_state_version(parent, version, budget).map_err(CompleteExecutionError::State)?;
    let required = estimate_clone_reservation(parent)?;
    if reserved_clone_bytes < required {
        return Err(CompleteExecutionError::CloneReservation {
            required,
            reserved: reserved_clone_bytes,
        });
    }
    for height in version.height.saturating_sub(255)..=version.height {
        if !parent.block_hashes.contains_key(&height) {
            return Err(CompleteExecutionError::State(StateError::MissingHistory(
                height,
            )));
        }
    }
    let mut cache = InMemoryDB::default();
    for (address, account) in &parent.accounts {
        let code = if account.code_hash == alloy_trie::KECCAK_EMPTY {
            Bytecode::default()
        } else {
            Bytecode::new_legacy(
                parent
                    .codes
                    .get(&account.code_hash)
                    .ok_or(CompleteExecutionError::State(StateError::MissingCode(
                        account.code_hash,
                    )))?
                    .clone(),
            )
        };
        cache.insert_account_info(
            *address,
            AccountInfo {
                balance: account.balance,
                nonce: account.nonce,
                code_hash: account.code_hash,
                code: Some(code),
                ..Default::default()
            },
        );
        for (slot, value) in &account.storage {
            cache
                .insert_account_storage(*address, *slot, *value)
                .expect("complete oracle EmptyDB is infallible");
        }
    }
    for (hash, code) in &parent.codes {
        cache
            .cache
            .contracts
            .insert(*hash, Bytecode::new_legacy(code.clone()));
    }
    for (height, hash) in &parent.block_hashes {
        cache.cache.block_hashes.insert(U256::from(*height), hash.0);
    }
    Ok(cache)
}
