// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::decode_native_validator::decode_native_validator;
use crate::consensus::certificates::ClassicalValidator;
use anyhow::{Result, ensure};
use serde_json::Value;
pub fn decode_native_validators(value: &Value) -> Result<Vec<ClassicalValidator>> {
    let values = value
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("native validator array missing"))?;
    ensure!(
        !values.is_empty() && values.len() <= super::types::MAXIMUM_DECODED_VALIDATORS,
        "native validator count bound"
    );
    let mut addresses = std::collections::BTreeSet::new();
    values
        .iter()
        .map(|value| decode_native_validator(value, &mut addresses))
        .collect()
}
