// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::{BTreeMap, BTreeSet};

use alloy_primitives::{KECCAK256_EMPTY, keccak256};
use alloy_trie::{EMPTY_ROOT_HASH, Nibbles, proof::verify_proof};

use super::{AccountProof, ProofLimits, StateProofError, StorageProof, estimate_proof_reservation};
use crate::state::commitments::{build_account_trie, build_storage_trie, build_trie_account};
use crate::{Address, StateView, U256, view_state, view_version};

pub fn build_account_proof(
    view: &StateView,
    address: Address,
    slots: &[U256],
    limits: &ProofLimits,
    reserved_rebuild_bytes: usize,
) -> Result<AccountProof, StateProofError> {
    if slots.len() > limits.maximum_requested_slots
        || slots.len() > 256
        || slots.iter().collect::<BTreeSet<_>>().len() != slots.len()
    {
        return Err(StateProofError::Limit("requested storage keys"));
    }
    let required = estimate_proof_reservation(view, slots.len())?;
    if required > limits.maximum_rebuild_bytes || required > reserved_rebuild_bytes {
        return Err(StateProofError::Reservation {
            required,
            reserved: reserved_rebuild_bytes.min(limits.maximum_rebuild_bytes),
        });
    }
    let state = view_state(view);
    let key = Nibbles::unpack(keccak256(address));
    let (root, nodes) = build_account_trie(&state.accounts, vec![key]);
    if root != view_version(view).evm_root.0 {
        return Err(StateProofError::RootMismatch);
    }
    let account_proof: Vec<_> = nodes
        .matching_nodes_sorted(&key)
        .into_iter()
        .map(|(_, node)| node)
        .collect();
    let account = state.accounts.get(&address);
    let expected = account.map(|account| alloy_rlp::encode(build_trie_account(account)));
    verify_proof(root, key, expected, &account_proof).map_err(|_| StateProofError::InvalidProof)?;
    let storage_keys: Vec<_> = slots
        .iter()
        .map(|slot| Nibbles::unpack(keccak256(slot.to_be_bytes::<32>())))
        .collect();
    let empty_storage = BTreeMap::new();
    let storage = account.map_or(&empty_storage, |account| &account.storage);
    let (storage_root, storage_nodes) = build_storage_trie(storage, storage_keys.clone());
    let mut storage_proof = Vec::with_capacity(slots.len());
    let mut bytes = account_proof
        .iter()
        .try_fold(0_usize, |sum, node| sum.checked_add(node.len()))
        .ok_or(StateProofError::ArithmeticOverflow)?;
    if bytes > limits.maximum_proof_bytes {
        return Err(StateProofError::Limit("proof response bytes"));
    }
    for (slot, key) in slots.iter().zip(storage_keys) {
        let value = account
            .and_then(|account| account.storage.get(slot))
            .copied()
            .unwrap_or(U256::ZERO);
        let proof: Vec<_> = storage_nodes
            .matching_nodes_sorted(&key)
            .into_iter()
            .map(|(_, node)| node)
            .collect();
        bytes = proof
            .iter()
            .try_fold(bytes, |sum, node| sum.checked_add(node.len()))
            .ok_or(StateProofError::ArithmeticOverflow)?;
        if bytes > limits.maximum_proof_bytes {
            return Err(StateProofError::Limit("proof response bytes"));
        }
        let expected = (!value.is_zero()).then(|| alloy_rlp::encode(value));
        verify_proof(storage_root, key, expected, &proof)
            .map_err(|_| StateProofError::InvalidProof)?;
        storage_proof.push(StorageProof {
            key: *slot,
            value,
            proof,
        });
    }
    Ok(AccountProof {
        height: view_version(view).height,
        state_root: view_version(view).evm_root,
        address,
        nonce: account.map_or(0, |account| account.nonce),
        balance: account.map_or(U256::ZERO, |account| account.balance),
        code_hash: account.map_or(KECCAK256_EMPTY, |account| account.code_hash),
        storage_root: if account.is_some() {
            storage_root
        } else {
            EMPTY_ROOT_HASH
        },
        account_proof,
        storage_proof,
    })
}
