// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_trie::TrieAccount;

use super::build_storage_trie;
use crate::StateAccount;

pub(crate) fn build_trie_account(account: &StateAccount) -> TrieAccount {
    TrieAccount::new(
        account.nonce,
        account.balance,
        build_storage_trie(&account.storage, Vec::new()).0,
        account.code_hash,
    )
}
