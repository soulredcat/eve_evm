// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    RpcContext,
    errors::rpc_error,
    source::{AppliedRpcConfig, RpcContextConfiguration, RpcStateSource, initialize_rpc_context},
};
use crate::{mempool::MempoolHandle, sync::applied::AppliedReader};
use jsonrpsee::types::ErrorObjectOwned;
use std::sync::Arc;

/// Attach an actual charged immutable RAM source. No durable repository or producer is fabricated.
pub(crate) fn create_applied_rpc_context(
    reader: AppliedReader,
    pool: MempoolHandle,
    config: AppliedRpcConfig,
) -> Result<Arc<RpcContext>, ErrorObjectOwned> {
    let buffer_kib = u32::try_from(config.buffer_bytes.div_ceil(1024))
        .map_err(|_| rpc_error(-32005, "RPC buffer accounting overflow"))?;
    if config.buffer_bytes == 0
        || !config.buffer_bytes.is_multiple_of(1024)
        || config.maximum_simulation_memory_bytes == 0
        || config.maximum_simulation_memory_bytes > 32 * 1_048_576
    {
        return Err(rpc_error(-32005, "invalid applied RPC resource profile"));
    }
    let simulation_memory_bytes = u64::try_from(config.maximum_simulation_memory_bytes)
        .map_err(|_| rpc_error(-32005, "simulation memory accounting overflow"))?;
    initialize_rpc_context(
        RpcStateSource::Applied { reader },
        pool,
        RpcContextConfiguration {
            state_budget: config.state_budget,
            zone: config.zone,
            buffer_kib,
            producer_kib: 0,
            simulation_memory_bytes,
        },
    )
}
