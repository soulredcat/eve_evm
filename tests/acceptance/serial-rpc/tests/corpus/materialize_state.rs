// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::FixtureAccount;
use alloy_primitives::{Address, U256, keccak256};
use revm::{
    database::InMemoryDB,
    state::{AccountInfo, Bytecode},
};
use std::collections::BTreeMap;

pub fn materialize_state(accounts: &BTreeMap<Address, FixtureAccount>) -> InMemoryDB {
    let mut database = InMemoryDB::default();
    for (address, account) in accounts {
        let code = Bytecode::new_raw(account.code.clone());
        database.insert_account_info(
            *address,
            AccountInfo::new(
                account.balance,
                account.nonce.try_into().expect("Fixture nonce fits u64"),
                keccak256(&account.code),
                code,
            ),
        );
        for (slot, value) in &account.storage {
            if *value != U256::ZERO {
                database
                    .insert_account_storage(*address, *slot, *value)
                    .unwrap();
            }
        }
    }
    database
}
