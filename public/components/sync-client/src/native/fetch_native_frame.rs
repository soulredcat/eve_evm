// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::NativeRpcConfig;
use anyhow::Result;
use eve_finality_verifier::NativeDataFrame;
use std::time::Instant;

/// Actual native block and matching certificate under one configured total deadline.
/// The caller keeps its actual working lease; all returned data remains untrusted.
pub fn fetch_native_frame(
    config: NativeRpcConfig,
    height: u64,
    reserved_bytes: usize,
) -> Result<NativeDataFrame> {
    let deadline = Instant::now()
        .checked_add(config.timeout)
        .ok_or_else(|| anyhow::anyhow!("SYNC_NATIVE_DEADLINE"))?;
    super::fetch_native_frame_before::fetch_native_frame_before(
        config,
        height,
        reserved_bytes,
        deadline,
    )
}
