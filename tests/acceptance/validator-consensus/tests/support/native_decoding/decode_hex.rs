// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use serde_json::Value;
pub(super) fn decode_hex(value: &Value, width: usize, empty: bool) -> Result<Vec<u8>> {
    let text = value
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("native hex string missing"))?;
    ensure!(
        text.len() == width * 2 || (empty && text.is_empty()),
        "native hex width"
    );
    hex::decode(text).map_err(|_| anyhow::anyhow!("invalid native hex"))
}
