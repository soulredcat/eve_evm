// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use serde_json::Value;

pub(super) fn decode_observed_height(value: &Value) -> Result<i64> {
    let encoded = value.as_str().context("B3_SUBMIT_OBSERVATION_HEIGHT")?;
    ensure!(
        !encoded.is_empty()
            && encoded.len() <= 19
            && (encoded == "0" || !encoded.starts_with('0'))
            && encoded.bytes().all(|byte| byte.is_ascii_digit()),
        "B3_SUBMIT_OBSERVATION_HEIGHT"
    );
    encoded
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("B3_SUBMIT_OBSERVATION_HEIGHT"))
}
