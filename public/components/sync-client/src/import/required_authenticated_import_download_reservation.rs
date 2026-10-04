// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{NativeRpcConfig, required_state_delta_download_reservation};
use anyhow::Result;

/// Initial transport/body charge. Actual-count materialization is leased separately
/// by the assembly callback; this value never represents its whole peak or RSS.
pub fn required_authenticated_import_download_reservation(
    config: NativeRpcConfig,
) -> Result<usize> {
    required_state_delta_download_reservation(config)
}
