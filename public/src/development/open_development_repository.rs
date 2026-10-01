// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    read_development_spec::read_development_spec, resolve_data_directory::resolve_data_directory,
};
use crate::runtime::DevelopmentPublicConfig;
use anyhow::{Result, ensure};
use eve_state::initialize_development_state;
use eve_storage::state::{
    HistoryReadBudget, StateRepository, development_state_storage_budget, ensure_history_index,
    open_state_repository,
};
pub(crate) fn open_development_repository(
    config: &DevelopmentPublicConfig,
) -> Result<StateRepository> {
    ensure!(
        config.mode == "DEV_ALL_IN_ONE" && config.acknowledge_unsafe_development,
        "local producer requires explicit acknowledged DEV_ALL_IN_ONE; production cannot activate this profile"
    );
    ensure!(
        config.block_interval_ms >= 10 && config.block_interval_ms <= 60_000,
        "block interval must be between10 and60000ms"
    );
    ensure!(
        config.allow_external_bind
            || (config.http_address.ip().is_loopback() && config.ws_address.ip().is_loopback()),
        "external binding requires explicit opt in"
    );
    let path = resolve_data_directory(&config.root, &config.data)?;
    let budget = development_state_storage_budget();
    let genesis =
        initialize_development_state(&read_development_spec(&config.genesis)?, &budget.logical)
            .map_err(|e| anyhow::anyhow!("genesis initialization rejected: {e:?}"))?;
    let mut store = open_state_repository(&path, &genesis, budget)?;
    super::persist_node_metadata::persist_node_metadata(
        &path,
        eve_node_policy::ZoneId(config.zone_id),
        &genesis.target.identity,
    )?;
    let limits = HistoryReadBudget {
        maximum_block_bytes: 16 * 1_048_576,
        maximum_rebuild_blocks: 128,
        maximum_index_batch_bytes: 16 * 1_048_576,
    };
    while !ensure_history_index(&mut store, limits)?.complete {}
    Ok(store)
}
