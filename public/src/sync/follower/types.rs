// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_sync_client::NativeRpcConfig;

/// Explicit loopback validator source and local download/operator limits.
#[derive(Clone, Copy)]
pub struct ValidatorFollowerSource {
    pub rpc: NativeRpcConfig,
    pub maximum_chunk_bytes: u32,
}
