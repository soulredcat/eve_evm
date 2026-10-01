// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{PoolError, types::PoolState};
use alloy_primitives::U256;
use eve_evm::check_transaction_admission;
use eve_protocol_config::headers::derive_next_base_fee;
use std::collections::BTreeMap;
pub(crate) fn revalidate_pool(state: &mut PoolState) -> Result<(), PoolError> {
    let fee = derive_next_base_fee(&state.head.block.header)
        .map_err(|e| PoolError(format!("base fee: {e:?}")))?;
    let previous = std::mem::take(&mut state.entries);
    for (sender, entries) in previous {
        let account = state.head.state.accounts.get(&sender);
        let mut retained = BTreeMap::new();
        let mut reserved = U256::ZERO;
        for (nonce, mut entry) in entries {
            let admission = check_transaction_admission(
                &entry.validated,
                account,
                fee,
                state.head.block.header.gas_limit,
            );
            let mut retain = false;
            if let Ok(admission) = admission {
                let sum = reserved.checked_add(admission.maximum_upfront_cost);
                if admission.nonce >= admission.state_nonce
                    && admission.nonce - admission.state_nonce <= state.limits.maximum_nonce_gap
                    && sum.is_some_and(|sum| {
                        sum <= account.map_or(U256::ZERO, |account| account.balance)
                    })
                {
                    reserved = sum.unwrap_or(U256::MAX);
                    entry.admission = admission;
                    retain = true;
                }
            }
            if retain {
                retained.insert(nonce, entry);
            } else {
                state.hashes.remove(&entry.admission.hash);
                state.bytes -= entry.raw.len();
            }
        }
        if !retained.is_empty() {
            state.entries.insert(sender, retained);
        }
    }
    Ok(())
}
