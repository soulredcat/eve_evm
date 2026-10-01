// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::RpcContext;
use crate::mempool::MempoolHandle;
use eve_storage::state::{HistoryReadBudget, StateRepository, create_state_service, state_reader};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, atomic::AtomicBool},
};
use tokio::sync::{Semaphore, broadcast};
pub(crate) fn create_rpc_context(
    repository: &StateRepository,
    pool: MempoolHandle,
    state_budget: eve_state::StateBudget,
    zone: eve_node_policy::ZoneId,
) -> Arc<RpcContext> {
    let (events, _) = broadcast::channel(32);
    let bytes = Arc::new(Semaphore::new(512 * 1024));
    let producer_bytes = Arc::clone(&bytes)
        .try_acquire_many_owned(256 * 1024)
        .expect("new global budget reserves producer quarter-gigabyte");
    Arc::new(RpcContext {
        zone,
        producer_bytes,
        service: create_state_service(state_reader(repository)),
        reader: state_reader(repository),
        pool,
        state_budget,
        history_budget: HistoryReadBudget {
            maximum_block_bytes: 16 * 1_048_576,
            maximum_rebuild_blocks: 128,
            maximum_index_batch_bytes: 16 * 1_048_576,
        },
        active: Arc::new(Semaphore::new(128)),
        signatures: Arc::new(Semaphore::new(2)),
        simulations: Arc::new(Semaphore::new(8)),
        histories: Arc::new(Semaphore::new(4)),
        proofs: Arc::new(Semaphore::new(2)),
        bytes,
        subscriptions: Arc::new(Semaphore::new(256)),
        events,
        historical_cache: Mutex::new(BTreeMap::new()),
        healthy: AtomicBool::new(true),
    })
}
