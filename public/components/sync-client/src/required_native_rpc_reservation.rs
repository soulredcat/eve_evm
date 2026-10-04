// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::NativeRpcConfig;
use anyhow::{Context, Result};

/// Conservative raw/JSON working reservation, not allocator or RSS containment.
/// Each caller must acquire and retain actual resource leases through returned Value lifetime.
pub fn required_native_rpc_reservation(config: NativeRpcConfig) -> Result<usize> {
    config
        .maximum_response_bytes
        .checked_mul(256)
        .and_then(|bytes| bytes.checked_add(config.maximum_request_bytes.checked_mul(2)?))
        .and_then(|bytes| bytes.checked_add(65_536))
        .context("SYNC_RPC_RESERVATION_OVERFLOW")
}
