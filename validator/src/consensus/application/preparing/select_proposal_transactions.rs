// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::transaction_wire_size::transaction_wire_size;
use alloy_primitives::{Address, U256};
use anyhow::{Result, ensure};
use eve_evm::{check_transaction_admission, decode_signed_transaction};
use eve_protocol_config::headers::derive_next_base_fee;
use eve_state::StateCommit;
use std::collections::BTreeMap;

/// Conservative proposer policy; supplied remote proposals are never filtered by it.
pub(super) fn select_proposal_transactions(
    parent: &StateCommit,
    supplied: &[Vec<u8>],
    maximum_bytes: i64,
) -> Result<Vec<Vec<u8>>> {
    ensure!(
        (0..=4_194_304).contains(&maximum_bytes) && supplied.len() <= 100_000,
        "native prepare input bounds"
    );
    let base_fee = derive_next_base_fee(&parent.block.header)
        .map_err(|_| anyhow::anyhow!("invalid prepared base-fee context"))?;
    let mut senders: BTreeMap<Address, (u64, U256)> = BTreeMap::new();
    let mut selected = Vec::new();
    let mut bytes = 0_usize;
    let mut gas = 0_u64;
    for raw in supplied {
        if selected.len() >= 30_000_000 / 21_000 {
            break;
        }
        let transaction =
            match decode_signed_transaction(raw, parent.target.identity.evm_chain_id, 131_072) {
                Ok(transaction) => transaction,
                Err(_) => continue,
            };
        let admission = match check_transaction_admission(
            &transaction,
            parent.state.accounts.get(&transaction.sender()),
            base_fee,
            parent.block.header.gas_limit,
        ) {
            Ok(admission) => admission,
            Err(_) => continue,
        };
        let account = parent.state.accounts.get(&admission.sender);
        let (nonce, reserved) = senders
            .get(&admission.sender)
            .copied()
            .unwrap_or((admission.state_nonce, U256::ZERO));
        if admission.nonce != nonce {
            continue;
        }
        let Some(next_nonce) = nonce.checked_add(1) else {
            continue;
        };
        let Some(next_reserved) = reserved.checked_add(admission.maximum_upfront_cost) else {
            continue;
        };
        if next_reserved > account.map_or(U256::ZERO, |account| account.balance) {
            continue;
        }
        let Some(next_gas) = gas.checked_add(admission.gas_limit) else {
            continue;
        };
        let next_bytes = bytes
            .checked_add(transaction_wire_size(raw)?)
            .ok_or_else(|| anyhow::anyhow!("prepared byte count overflow"))?;
        if next_gas > parent.block.header.gas_limit
            || u64::try_from(next_bytes)? > u64::try_from(maximum_bytes)?
        {
            continue;
        }
        senders.insert(admission.sender, (next_nonce, next_reserved));
        bytes = next_bytes;
        gas = next_gas;
        selected.push(raw.clone());
    }
    Ok(selected)
}
