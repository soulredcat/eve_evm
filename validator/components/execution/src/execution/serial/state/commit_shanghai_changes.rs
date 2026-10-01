// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use revm::{
    DatabaseCommit,
    database::{DbAccount, InMemoryDB},
    state::EvmState,
};

/// EIP-161 clearing at EACH transaction boundary, before the next EVM lookup.
/// CacheDB's generic commit keeps touched empty accounts; Shanghai removes them.
/// Tombstones preserve absence for complete-state projection and later recreation.
pub(crate) fn commit_shanghai_changes(database: &mut InMemoryDB, changes: EvmState) {
    let mut cleared: Vec<_> = changes
        .iter()
        .filter(|(_, account)| account.is_touched() && account.info.is_empty())
        .map(|(address, _)| *address)
        .collect();
    cleared.sort_unstable();
    database.commit(changes);
    for address in cleared {
        database
            .cache
            .accounts
            .insert(address, DbAccount::new_not_existing());
    }
}
