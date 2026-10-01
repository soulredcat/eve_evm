// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::StateAccount;
use revm::database::DbAccount;

pub(crate) fn project_revm_account(account: &DbAccount) -> Option<StateAccount> {
    let info = account.info()?;
    Some(StateAccount {
        nonce: info.nonce,
        balance: info.balance,
        code_hash: info.code_hash,
        storage: account
            .storage
            .iter()
            .filter_map(|(slot, value)| (!value.is_zero()).then_some((*slot, *value)))
            .collect(),
    })
}
