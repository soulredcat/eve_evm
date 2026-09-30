use std::collections::BTreeMap;

use alloy_primitives::{Address, B256};
use alloy_trie::{TrieAccount, root};
use eve_protocol_config::records::EvmStateRoot;

use crate::StateAccount;

/// Canonical world-state root; callers supply complete, validated logical accounts.
pub fn compute_evm_root(accounts: &BTreeMap<Address, StateAccount>) -> EvmStateRoot {
    EvmStateRoot(root::state_root_unhashed(accounts.iter().map(
        |(address, account)| {
            let storage = account.storage.iter().filter_map(|(slot, value)| {
                (!value.is_zero()).then_some((B256::from(slot.to_be_bytes::<32>()), *value))
            });
            (
                *address,
                TrieAccount::new(
                    account.nonce,
                    account.balance,
                    root::storage_root_unhashed(storage),
                    account.code_hash,
                ),
            )
        },
    )))
}
