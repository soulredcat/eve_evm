// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use serde_json::Value;

pub(super) fn validate_submission_height(result: &Value) -> Result<i64> {
    let encoded = result
        .get("height")
        .and_then(Value::as_str)
        .context("B3_SUBMIT_HEIGHT_MISSING")?;
    ensure!(
        !encoded.is_empty()
            && encoded.len() <= 19
            && !encoded.starts_with('0')
            && encoded.bytes().all(|byte| byte.is_ascii_digit()),
        "B3_SUBMIT_HEIGHT_INVALID"
    );
    let height = encoded
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("B3_SUBMIT_HEIGHT_INVALID"))?;
    ensure!(height > 0, "B3_SUBMIT_HEIGHT_INVALID");
    Ok(height)
}
