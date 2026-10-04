// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{MasterFollowerOptions, load_charged_master_genesis::load_charged_master_genesis};
use crate::sync::{follow_master_to_height, master_sync_development_config, open_master_follower};
use anyhow::{Result, ensure};
use eve_sync_client::{
    NativeRpcConfig, required_authenticated_import_download_reservation, validate_native_rpc_config,
};
use std::time::Duration;

pub fn run_master_follower_cli(options: MasterFollowerOptions) -> Result<()> {
    ensure!(
        options.mode == "MASTER_SYNC_ONLY" && options.acknowledge_unsafe_development,
        "MASTER_SYNC_ONLY requires explicit unsafe classical-development acknowledgement"
    );
    let rpc = NativeRpcConfig {
        address: options.native_address,
        timeout: Duration::from_millis(options.timeout_ms),
        maximum_request_bytes: 131_072,
        maximum_response_bytes: options.maximum_response_bytes,
    };
    validate_native_rpc_config(rpc)?;
    required_authenticated_import_download_reservation(rpc)?;
    let genesis = load_charged_master_genesis(&options.genesis)?;
    let mut config = master_sync_development_config(options.root, options.data);
    config.mode = options.mode;
    config.acknowledge_unsafe_development = options.acknowledge_unsafe_development;
    let mut follower = open_master_follower(config, &genesis.genesis)?;
    let status = follow_master_to_height(
        &mut follower,
        rpc,
        options.maximum_chunk_bytes,
        options.through_height,
    )?;
    println!("{}", serde_json::to_string(&status)?);
    Ok(())
}
