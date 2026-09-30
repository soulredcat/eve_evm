use alloy_primitives::{B256, U256};
use alloy_trie::{TrieAccount, root};
use revm::database::InMemoryDB;

/// Root of a complete EmptyDB-backed state, never a partial remote-state cache.
pub fn compute_state_root(state: &InMemoryDB) -> B256 {
    root::state_root_unhashed(
        state
            .cache
            .accounts
            .iter()
            .filter_map(|(address, account)| {
                let info = account.info()?;
                let storage = account.storage.iter().filter_map(|(key, value)| {
                    (*value != U256::ZERO).then_some((B256::from(key.to_be_bytes::<32>()), *value))
                });
                Some((
                    *address,
                    TrieAccount::new(
                        info.nonce,
                        info.balance,
                        root::storage_root_unhashed(storage),
                        info.code_hash,
                    ),
                ))
            }),
    )
}
