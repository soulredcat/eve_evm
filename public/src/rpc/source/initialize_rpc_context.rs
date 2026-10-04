// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RpcStateSource, types::RpcContextConfiguration};
use crate::{
    mempool::MempoolHandle,
    rpc::{RpcContext, errors::rpc_error},
};
use eve_storage::state::HistoryReadBudget;
use jsonrpsee::types::ErrorObjectOwned;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, atomic::AtomicBool},
};
use tokio::sync::{Semaphore, broadcast};

pub(in crate::rpc) fn initialize_rpc_context(
    source: RpcStateSource,
    pool: MempoolHandle,
    configuration: RpcContextConfiguration,
) -> Result<Arc<RpcContext>, ErrorObjectOwned> {
    let capacity = usize::try_from(configuration.buffer_kib)
        .map_err(|_| rpc_error(-32005, "RPC buffer capacity overflow"))?;
    if capacity == 0
        || capacity > Semaphore::MAX_PERMITS
        || configuration.producer_kib > configuration.buffer_kib
    {
        return Err(rpc_error(-32005, "invalid RPC resource profile"));
    }
    let (events, _) = broadcast::channel(32);
    let bytes = Arc::new(Semaphore::new(capacity));
    let producer_bytes = Arc::clone(&bytes)
        .try_acquire_many_owned(configuration.producer_kib)
        .map_err(|_| rpc_error(-32005, "RPC producer reservation unavailable"))?;
    Ok(Arc::new(RpcContext {
        zone: configuration.zone,
        producer_bytes,
        source,
        pool,
        state_budget: configuration.state_budget,
        simulation_memory_bytes: configuration.simulation_memory_bytes,
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
    }))
}
