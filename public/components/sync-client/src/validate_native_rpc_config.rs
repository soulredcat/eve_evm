// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::NativeRpcConfig;
use anyhow::{Result, ensure};
use std::time::Duration;

/// The same finite local transport policy applies before namespace creation and requests.
pub fn validate_native_rpc_config(config: NativeRpcConfig) -> Result<()> {
    ensure!(
        config.address.ip().is_loopback() && config.address.port() != 0,
        "SYNC_RPC_ENDPOINT"
    );
    ensure!(
        !config.timeout.is_zero() && config.timeout <= Duration::from_secs(30),
        "SYNC_RPC_TIMEOUT"
    );
    ensure!(
        config.maximum_request_bytes > 0
            && config.maximum_request_bytes <= 1_048_576
            && config.maximum_response_bytes > 0
            && config.maximum_response_bytes <= 32 * 1_048_576,
        "SYNC_RPC_LIMITS"
    );
    Ok(())
}
