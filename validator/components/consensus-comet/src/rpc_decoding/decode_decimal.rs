// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use serde_json::Value;
use std::str::FromStr;
pub(super) fn decode_decimal<T: FromStr>(value: &Value) -> Result<T> {
    let text = value
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("native decimal string missing"))?;
    ensure!(
        !text.is_empty()
            && text.len() <= 20
            && text
                .bytes()
                .enumerate()
                .all(|(index, byte)| byte.is_ascii_digit() || (index == 0 && byte == b'-')),
        "invalid native decimal syntax"
    );
    text.parse()
        .map_err(|_| anyhow::anyhow!("native decimal range"))
}
