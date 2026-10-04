// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::Value;
pub(super) fn decode_base64(value: &Value, maximum: usize, nullable: bool) -> Result<Vec<u8>> {
    if nullable && value.is_null() {
        return Ok(Vec::new());
    }
    let text = value
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("native base64 string missing"))?;
    ensure!(
        text.len() <= maximum.div_ceil(3) * 4,
        "native base64 encoded byte limit"
    );
    let bytes = STANDARD
        .decode(text)
        .map_err(|_| anyhow::anyhow!("invalid native base64"))?;
    ensure!(bytes.len() <= maximum, "native decoded byte limit");
    Ok(bytes)
}
