// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    PoolError,
    replacement_threshold::replacement_threshold,
    types::{PoolEntry, PoolState},
};
use alloy_primitives::{B256, Bytes, U256};
use eve_evm::{ValidatedTransaction, check_transaction_admission};
use eve_protocol_config::headers::derive_next_base_fee;
use std::time::Instant;
pub(crate) fn admit_transaction(
    state: &mut PoolState,
    raw: Bytes,
    validated: ValidatedTransaction,
    now: Instant,
) -> Result<B256, PoolError> {
    if raw.len() > 131_072 || alloy_primitives::keccak256(&raw) != validated.hash() {
        return Err(PoolError(
            "raw transaction size or validated identity mismatch".into(),
        ));
    }
    if state.hashes.contains_key(&validated.hash()) {
        return Ok(validated.hash());
    }
    let header = &state.head.block.header;
    let base_fee = derive_next_base_fee(header)
        .map_err(|e| PoolError(format!("invalid parent base fee: {e:?}")))?;
    let account = state.head.state.accounts.get(&validated.sender());
    let admission = check_transaction_admission(&validated, account, base_fee, header.gas_limit)
        .map_err(|e| PoolError(format!("admission rejected: {e:?}")))?;
    if admission.nonce < admission.state_nonce
        || admission.nonce - admission.state_nonce > state.limits.maximum_nonce_gap
    {
        return Err(PoolError("nonce is stale or exceeds future gap".into()));
    }
    let sender_entries = state.entries.get(&admission.sender);
    let previous = sender_entries.and_then(|entries| entries.get(&admission.nonce));
    if let Some(previous) = previous {
        if admission.transaction_type != previous.admission.transaction_type {
            return Err(PoolError("replacement transaction type mismatch".into()));
        }
        if admission.max_fee_per_gas < replacement_threshold(previous.admission.max_fee_per_gas)? {
            return Err(PoolError("replacement max fee increase too small".into()));
        }
        if let Some(previous_tip) = previous.admission.max_priority_fee_per_gas
            && admission.max_priority_fee_per_gas.unwrap_or(0)
                < replacement_threshold(previous_tip)?
        {
            return Err(PoolError(
                "replacement priority fee increase too small".into(),
            ));
        }
    } else if sender_entries.is_some_and(|entries| entries.len() >= state.limits.maximum_per_sender)
        || state.hashes.len() >= state.limits.maximum_transactions
    {
        return Err(PoolError(
            "mempool transaction count capacity exceeded".into(),
        ));
    }
    let reserved = sender_entries
        .into_iter()
        .flat_map(|entries| entries.values())
        .filter(|entry| entry.admission.nonce != admission.nonce)
        .try_fold(admission.maximum_upfront_cost, |sum, entry| {
            sum.checked_add(entry.admission.maximum_upfront_cost)
        })
        .ok_or_else(|| PoolError("sender upfront reservation overflow".into()))?;
    if reserved > account.map_or(U256::ZERO, |account| account.balance) {
        return Err(PoolError(
            "cumulative sender maximum-upfront balance reservation exceeded".into(),
        ));
    }
    let previous_bytes = previous.map_or(0, |entry| entry.raw.len());
    let next_bytes = state
        .bytes
        .checked_sub(previous_bytes)
        .and_then(|bytes| bytes.checked_add(raw.len()))
        .ok_or_else(|| PoolError("mempool byte accounting overflow".into()))?;
    if next_bytes > state.limits.maximum_bytes {
        return Err(PoolError("mempool raw byte capacity exceeded".into()));
    }
    let hash = admission.hash;
    let sender = admission.sender;
    let nonce = admission.nonce;
    let old = state.entries.entry(sender).or_default().insert(
        nonce,
        PoolEntry {
            raw,
            validated,
            admission,
            admitted_at: now,
        },
    );
    if let Some(old) = old {
        state.hashes.remove(&old.admission.hash);
    }
    state.hashes.insert(hash, (sender, nonce));
    state.bytes = next_bytes;
    Ok(hash)
}
