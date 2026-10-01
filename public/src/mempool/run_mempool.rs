// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    admit_transaction::admit_transaction,
    evict_expired::evict_expired,
    revalidate_pool::revalidate_pool,
    select_transactions::select_transactions,
    types::{PoolCommand, PoolState},
};
use std::time::Instant;
use tokio::sync::mpsc;
pub(crate) async fn run_mempool(mut state: PoolState, mut receiver: mpsc::Receiver<PoolCommand>) {
    while let Some(command) = receiver.recv().await {
        evict_expired(&mut state, Instant::now());
        match command {
            PoolCommand::Capture(response) => {
                let _ = response.send((
                    std::sync::Arc::clone(&state.head),
                    select_transactions(&state),
                ));
            }
            PoolCommand::Admit(raw, validated, response) => {
                let _ = response.send(admit_transaction(
                    &mut state,
                    raw,
                    *validated,
                    Instant::now(),
                ));
            }
            PoolCommand::Select(response) => {
                let _ = response.send(select_transactions(&state));
            }
            PoolCommand::Find(hash, response) => {
                let _ = response.send(
                    state
                        .hashes
                        .get(&hash)
                        .and_then(|(sender, nonce)| state.entries.get(sender)?.get(nonce))
                        .cloned(),
                );
            }
            PoolCommand::PendingNonce(sender, response) => {
                let mut nonce = state
                    .head
                    .state
                    .accounts
                    .get(&sender)
                    .map_or(0, |account| account.nonce);
                if let Some(entries) = state.entries.get(&sender) {
                    while entries.contains_key(&nonce) {
                        let Some(next) = nonce.checked_add(1) else {
                            break;
                        };
                        nonce = next;
                    }
                }
                let _ = response.send(nonce);
            }
            PoolCommand::Committed(head, response) => {
                state.head = head;
                let _ = response.send(revalidate_pool(&mut state));
            }
            PoolCommand::Evict(now, response) => {
                let _ = response.send(evict_expired(&mut state, now));
            }
        }
    }
}
