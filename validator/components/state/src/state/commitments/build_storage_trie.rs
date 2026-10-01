// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeMap;

use alloy_primitives::{B256, U256, keccak256};
use alloy_trie::{
    HashBuilder, Nibbles,
    proof::{ProofNodes, ProofRetainer},
};

/// Sole storage leaf/root builder, optionally retaining requested proof paths.
pub(crate) fn build_storage_trie(
    storage: &BTreeMap<U256, U256>,
    targets: Vec<Nibbles>,
) -> (B256, ProofNodes) {
    let mut leaves: Vec<_> = storage
        .iter()
        .filter(|(_, value)| !value.is_zero())
        .map(|(slot, value)| (keccak256(slot.to_be_bytes::<32>()), *value))
        .collect();
    leaves.sort_unstable_by_key(|(key, _)| *key);
    let mut builder = HashBuilder::default().with_proof_retainer(ProofRetainer::new(targets));
    for (key, value) in leaves {
        builder.add_leaf(Nibbles::unpack(key), &alloy_rlp::encode(value));
    }
    let root = builder.root();
    (root, builder.take_proof_nodes())
}
