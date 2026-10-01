// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::FixtureAccount;
use alloy_primitives::{Address, Bytes, U256};
use revm::database::InMemoryDB;
use std::collections::BTreeMap;

pub fn project_state(database: &InMemoryDB) -> BTreeMap<Address, FixtureAccount> {
    database
        .cache
        .accounts
        .iter()
        .filter_map(|(address, account)| {
            let info = account.info()?;
            let code = if let Some(code) = &info.code {
                Bytes::copy_from_slice(&code.original_bytes())
            } else if let Some(code) = database.cache.contracts.get(&info.code_hash) {
                Bytes::copy_from_slice(&code.original_bytes())
            } else {
                assert!(info.code_hash == alloy_trie::KECCAK_EMPTY || info.code_hash.is_zero());
                Bytes::new()
            };
            Some((
                *address,
                FixtureAccount {
                    nonce: U256::from(info.nonce),
                    balance: info.balance,
                    code,
                    storage: account
                        .storage
                        .iter()
                        .filter_map(|(key, value)| (!value.is_zero()).then_some((*key, *value)))
                        .collect(),
                },
            ))
        })
        .collect()
}
