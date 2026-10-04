// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::DurablePrefix;
use anyhow::{Context, Result, ensure};
use serde_json::Value;

pub(super) fn verify_recovered_startup(
    startup: &Value,
    durable: DurablePrefix,
    missing_tail_height: u64,
) -> Result<()> {
    let restored = startup["height"]
        .as_u64()
        .context("PUBLIC_TAIL_RESTORED_HEIGHT")?;
    ensure!(
        restored >= durable.height
            && restored < missing_tail_height
            && startup["authenticated_finality"] == true
            && startup["verification_mode"] == "AUTHENTICATED_IMPORT_CLASSICAL_DEV",
        "PUBLIC_TAIL_DURABLE_PREFIX_NOT_RESTORED_OR_NEW_TAIL_FABRICATED"
    );
    Ok(())
}
