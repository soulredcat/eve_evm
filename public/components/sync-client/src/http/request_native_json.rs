// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::NativeRpcConfig;
use anyhow::Result;
use serde_json::Value;
use std::time::Instant;

/// Bounded loopback development transport with one configured total deadline.
/// Actual caller leases must cover returned JSON; the response grants no authority.
pub fn request_native_json(
    config: NativeRpcConfig,
    path: &str,
    reserved_bytes: usize,
) -> Result<Value> {
    let deadline = Instant::now()
        .checked_add(config.timeout)
        .ok_or_else(|| anyhow::anyhow!("SYNC_RPC_DEADLINE"))?;
    super::request_native_json_before::request_native_json_before(
        config,
        path,
        reserved_bytes,
        deadline,
    )
}
