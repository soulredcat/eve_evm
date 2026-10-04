// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedReader;
use eve_node_policy::ZoneId;
use eve_state::StateBudget;
use eve_storage::state::{StateReader, StateService};

pub(crate) enum RpcStateSource {
    Durable {
        service: Box<StateService>,
        reader: Box<StateReader>,
    },
    Applied {
        reader: AppliedReader,
    },
}

/// Explicit local RPC resource profile; it grants no finality or fresh-head evidence.
pub(crate) struct AppliedRpcConfig {
    pub state_budget: StateBudget,
    pub zone: ZoneId,
    pub buffer_bytes: u64,
    pub maximum_simulation_memory_bytes: usize,
}

pub(in crate::rpc) struct RpcContextConfiguration {
    pub state_budget: StateBudget,
    pub zone: ZoneId,
    pub buffer_kib: u32,
    pub producer_kib: u32,
    pub simulation_memory_bytes: u64,
}
