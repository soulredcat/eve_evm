// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use serde_json::Value;

pub(super) fn validate_submission_hash(result: &Value, expected_hash: &[u8; 32]) -> Result<()> {
    let encoded = result
        .get("hash")
        .and_then(Value::as_str)
        .context("B3_SUBMIT_HASH_MISSING")?;
    let mut actual = [0; 32];
    hex::decode_to_slice(encoded, &mut actual)
        .map_err(|_| anyhow::anyhow!("B3_SUBMIT_HASH_FORMAT"))?;
    ensure!(actual == *expected_hash, "B3_SUBMIT_HASH_MISMATCH");
    Ok(())
}
