// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::constants::{EMPTY_OMMER_ROOT_HASH, EMPTY_WITHDRAWALS};
use alloy_primitives::{B64, U256};
use alloy_trie::root::ordered_trie_root_encoded;

use super::{validate_block_payload, validate_version_metadata};
use crate::{BlockPayload, StateBudget, StateError, StateVersion};

/// Canonical retained metadata only: not execution, content_digest, or finality proof.
pub fn validate_retained_block(
    version: &StateVersion,
    parent: Option<&StateVersion>,
    block: &BlockPayload,
    budget: &StateBudget,
) -> Result<(), StateError> {
    validate_version_metadata(version)?;
    validate_block_payload(block, &version.identity, budget)?;
    let header = &block.header;
    if header.number != version.height
        || header.timestamp != version.timestamp
        || header.state_root != version.evm_root.0
        || header.hash_slow() != version.execution_hash.0
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
        || ordered_trie_root_encoded(&block.transactions) != header.transactions_root
        || ordered_trie_root_encoded(&block.receipts) != header.receipts_root
    {
        return Err(StateError::CommitMismatch);
    }
    let mut extra = version.identity.protocol_version.to_be_bytes().to_vec();
    extra.extend_from_slice(&version.identity.genesis.0.as_slice()[..28]);
    if header.extra_data.as_ref() != extra.as_slice() {
        return Err(StateError::CommitMismatch);
    }
    match parent {
        None if version.height == 0
            && header.parent_hash.is_zero()
            && block.transactions.is_empty() => {}
        Some(parent) => {
            validate_version_metadata(parent)?;
            if parent.identity != version.identity
                || parent.height.checked_add(1) != Some(version.height)
                || header.parent_hash != parent.execution_hash.0
                || parent.timestamp > version.timestamp
            {
                return Err(StateError::ParentMismatch);
            }
        }
        _ => return Err(StateError::ParentMismatch),
    }
    Ok(())
}
