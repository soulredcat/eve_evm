// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::errors::rpc_error;
use alloy_primitives::U256;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn parse_quantity(value: &Value) -> Result<U256, ErrorObjectOwned> {
    let text = value
        .as_str()
        .ok_or_else(|| rpc_error(-32602, "quantity must be a hex string"))?;
    let hex = text
        .strip_prefix("0x")
        .ok_or_else(|| rpc_error(-32602, "quantity requires0x"))?;
    if hex.is_empty()
        || hex.len() > 64
        || (hex.len() > 1 && hex.starts_with('0'))
        || !hex.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err(rpc_error(-32602, "noncanonical hex quantity"));
    }
    U256::from_str_radix(hex, 16).map_err(|_| rpc_error(-32602, "invalid quantity"))
}
