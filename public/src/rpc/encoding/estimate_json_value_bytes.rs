// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::estimate_json_string_bytes::estimate_json_string_bytes;
use crate::rpc::errors::rpc_error;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn estimate_json_value_bytes(
    value: &Value,
    maximum: usize,
) -> Result<usize, ErrorObjectOwned> {
    let mut bytes = match value {
        Value::Null => 4,
        Value::Bool(true) => 4,
        Value::Bool(false) => 5,
        Value::Number(number) => number.to_string().len(),
        Value::String(string) => return estimate_json_string_bytes(string, maximum),
        _ => 2,
    };
    match value {
        Value::Array(values) => {
            for (index, item) in values.iter().enumerate() {
                bytes = bytes
                    .checked_add(usize::from(index > 0))
                    .ok_or_else(|| rpc_error(-32005, "JSON accounting overflow"))?;
                bytes = bytes
                    .checked_add(estimate_json_value_bytes(
                        item,
                        maximum.saturating_sub(bytes),
                    )?)
                    .ok_or_else(|| rpc_error(-32005, "JSON accounting overflow"))?;
            }
        }
        Value::Object(values) => {
            for (index, (key, item)) in values.iter().enumerate() {
                bytes = bytes
                    .checked_add(usize::from(index > 0) + 1)
                    .ok_or_else(|| rpc_error(-32005, "JSON accounting overflow"))?;
                bytes = bytes
                    .checked_add(estimate_json_string_bytes(
                        key,
                        maximum.saturating_sub(bytes),
                    )?)
                    .ok_or_else(|| rpc_error(-32005, "JSON accounting overflow"))?;
                bytes = bytes
                    .checked_add(estimate_json_value_bytes(
                        item,
                        maximum.saturating_sub(bytes),
                    )?)
                    .ok_or_else(|| rpc_error(-32005, "JSON accounting overflow"))?;
            }
        }
        _ => {}
    }
    if bytes > maximum {
        return Err(rpc_error(-32005, "JSON output byte capacity exceeded"));
    }
    Ok(bytes)
}
