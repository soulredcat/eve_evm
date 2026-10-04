// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    source::{RpcContextConfiguration, RpcStateSource, initialize_rpc_context},
    types::RpcContext,
};
use crate::mempool::MempoolHandle;
use eve_storage::state::{StateRepository, create_state_service, state_reader};
use std::sync::Arc;
pub(crate) fn create_rpc_context(
    repository: &StateRepository,
    pool: MempoolHandle,
    state_budget: eve_state::StateBudget,
    zone: eve_node_policy::ZoneId,
) -> Arc<RpcContext> {
    initialize_rpc_context(
        RpcStateSource::Durable {
            service: Box::new(create_state_service(state_reader(repository))),
            reader: Box::new(state_reader(repository)),
        },
        pool,
        RpcContextConfiguration {
            state_budget,
            zone,
            buffer_kib: 512 * 1024,
            producer_kib: 256 * 1024,
            simulation_memory_bytes: 32 * 1_048_576,
        },
    )
    .expect("fixed development RPC profile reserves producer quarter-gigabyte")
}
