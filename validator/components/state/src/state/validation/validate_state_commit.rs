use alloy_consensus::constants::{EMPTY_OMMER_ROOT_HASH, EMPTY_WITHDRAWALS};
use alloy_primitives::{B64, U256};
use alloy_trie::root::ordered_trie_root_encoded;

use super::{
    build_state_version, validate_block_payload, validate_state_identity, validate_state_version,
};
use crate::{StateBudget, StateCommit, StateError};

pub fn validate_state_commit(commit: &StateCommit, budget: &StateBudget) -> Result<(), StateError> {
    validate_state_version(&commit.state, &commit.target, budget)?;
    validate_block_payload(&commit.block, &commit.target.identity, budget)?;
    let header = &commit.block.header;
    if build_state_version(&commit.state, header, budget)? != commit.target
        || header.ommers_hash != EMPTY_OMMER_ROOT_HASH
        || header.difficulty != U256::ZERO
        || header.nonce != B64::ZERO
        || header.withdrawals_root != Some(EMPTY_WITHDRAWALS)
        || header.base_fee_per_gas.is_none_or(|fee| fee == 0)
        || header.gas_limit != 30_000_000
        || header.gas_used > header.gas_limit
        || header.blob_gas_used.is_some()
        || header.excess_blob_gas.is_some()
        || header.parent_beacon_block_root.is_some()
        || header.requests_hash.is_some()
        || header.block_access_list_hash.is_some()
        || header.slot_number.is_some()
        || commit.block.transactions.len() != commit.block.receipts.len()
        || ordered_trie_root_encoded(&commit.block.transactions) != header.transactions_root
        || ordered_trie_root_encoded(&commit.block.receipts) != header.receipts_root
    {
        return Err(StateError::CommitMismatch);
    }
    let mut extra = commit
        .target
        .identity
        .protocol_version
        .to_be_bytes()
        .to_vec();
    extra.extend_from_slice(&commit.target.identity.genesis.0.as_slice()[..28]);
    if header.extra_data.as_ref() != extra.as_slice() {
        return Err(StateError::CommitMismatch);
    }
    match &commit.parent {
        None if commit.target.height == 0
            && header.parent_hash.is_zero()
            && commit.block.transactions.is_empty() => {}
        Some(parent) => {
            validate_state_identity(&parent.identity)?;
            if parent.identity != commit.target.identity
                || parent.height.checked_add(1) != Some(commit.target.height)
                || header.parent_hash != parent.execution_hash.0
                || parent.timestamp > commit.target.timestamp
                || commit.state.block_hashes.get(&parent.height) != Some(&parent.execution_hash)
            {
                return Err(StateError::ParentMismatch);
            }
        }
        _ => return Err(StateError::ParentMismatch),
    }
    Ok(())
}
