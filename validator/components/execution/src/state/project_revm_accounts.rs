// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::project_revm_account::project_revm_account;
use alloy_primitives::Address;
use eve_state::StateAccount;
use revm::database::InMemoryDB;
use std::collections::BTreeMap;

pub(crate) fn project_revm_accounts(state: &InMemoryDB) -> BTreeMap<Address, StateAccount> {
    state
        .cache
        .accounts
        .iter()
        .filter_map(|(address, account)| {
            project_revm_account(account).map(|account| (*address, account))
        })
        .collect()
}
