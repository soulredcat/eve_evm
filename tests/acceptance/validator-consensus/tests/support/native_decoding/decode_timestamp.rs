// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use serde_json::Value;
use std::str::FromStr;
pub(super) fn decode_timestamp(value: &Value) -> Result<prost_types::Timestamp> {
    let text = value
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("native timestamp missing"))?;
    ensure!(text.len() <= 64, "native timestamp byte limit");
    prost_types::Timestamp::from_str(text).map_err(|_| anyhow::anyhow!("invalid native timestamp"))
}
