// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    PoolError,
    types::{PoolEntry, PoolState},
};
use eve_protocol_config::headers::derive_next_base_fee;
use std::collections::BTreeMap;
pub(crate) fn select_transactions(state: &PoolState) -> Result<Vec<PoolEntry>, PoolError> {
    let fee = derive_next_base_fee(&state.head.block.header)
        .map_err(|e| PoolError(format!("base fee: {e:?}")))?;
    let mut nonces: BTreeMap<_, _> = state
        .entries
        .keys()
        .map(|sender| {
            (
                *sender,
                state
                    .head
                    .state
                    .accounts
                    .get(sender)
                    .map_or(0, |account| account.nonce),
            )
        })
        .collect();
    let mut output = Vec::new();
    let mut gas = 0_u64;
    let mut raw_bytes = 0_usize;
    loop {
        let candidate = nonces
            .iter()
            .filter_map(|(sender, nonce)| state.entries.get(sender)?.get(nonce))
            .filter(|entry| {
                gas.checked_add(entry.admission.gas_limit)
                    .is_some_and(|sum| sum <= state.head.block.header.gas_limit)
                    && raw_bytes
                        .checked_add(entry.raw.len())
                        .is_some_and(|sum| sum <= 1_048_576)
                    && entry.admission.max_fee_per_gas >= u128::from(fee)
            })
            .max_by(|left, right| {
                let tip = |entry: &PoolEntry| {
                    entry
                        .admission
                        .max_priority_fee_per_gas
                        .unwrap_or(u128::MAX)
                        .min(entry.admission.max_fee_per_gas - u128::from(fee))
                };
                tip(left)
                    .cmp(&tip(right))
                    .then_with(|| right.admission.hash.cmp(&left.admission.hash))
            });
        let Some(candidate) = candidate else {
            break;
        };
        gas += candidate.admission.gas_limit;
        raw_bytes += candidate.raw.len();
        let Some(next) = candidate.admission.nonce.checked_add(1) else {
            break;
        };
        nonces.insert(candidate.admission.sender, next);
        output.push(candidate.clone());
    }
    Ok(output)
}
