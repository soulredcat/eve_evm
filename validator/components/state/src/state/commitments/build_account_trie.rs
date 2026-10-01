// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeMap;

use alloy_primitives::{Address, B256, keccak256};
use alloy_trie::{
    HashBuilder, Nibbles,
    proof::{ProofNodes, ProofRetainer},
};

use super::build_trie_account;
use crate::StateAccount;

/// Sole account leaf/root builder, optionally retaining requested proof paths.
pub(crate) fn build_account_trie(
    accounts: &BTreeMap<Address, StateAccount>,
    targets: Vec<Nibbles>,
) -> (B256, ProofNodes) {
    let mut leaves: Vec<_> = accounts
        .iter()
        .map(|(address, account)| (keccak256(address), build_trie_account(account)))
        .collect();
    leaves.sort_unstable_by_key(|(key, _)| *key);
    let mut builder = HashBuilder::default().with_proof_retainer(ProofRetainer::new(targets));
    for (key, account) in leaves {
        builder.add_leaf(Nibbles::unpack(key), &alloy_rlp::encode(account));
    }
    let root = builder.root();
    (root, builder.take_proof_nodes())
}
