// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{NativeRpcConfig, required_native_rpc_reservation};
use anyhow::{Context, Result};
use eve_state::MAXIMUM_STATE_DELTA_PAYLOAD_BYTES;

/// One complete raw body plus bounded response/chunk/reencoding copies. This
/// does not cover later journal materialization or candidate execution/import.
pub fn required_state_delta_download_reservation(config: NativeRpcConfig) -> Result<usize> {
    required_native_rpc_reservation(config)?
        .checked_add(
            MAXIMUM_STATE_DELTA_PAYLOAD_BYTES
                .checked_mul(3)
                .context("SYNC_DELTA_RESERVATION_OVERFLOW")?,
        )
        .context("SYNC_DELTA_RESERVATION_OVERFLOW")
}
