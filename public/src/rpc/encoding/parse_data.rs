// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::errors::rpc_error;
use alloy_primitives::Bytes;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn parse_data(value: &Value, maximum: usize) -> Result<Bytes, ErrorObjectOwned> {
    let text = value
        .as_str()
        .ok_or_else(|| rpc_error(-32602, "data must be a hex string"))?;
    let encoded = text
        .strip_prefix("0x")
        .ok_or_else(|| rpc_error(-32602, "data requires0x"))?;
    if !encoded.len().is_multiple_of(2) || encoded.len() / 2 > maximum {
        return Err(rpc_error(-32602, "malformed or oversized hex data"));
    }
    hex::decode(encoded)
        .map(Bytes::from)
        .map_err(|_| rpc_error(-32602, "invalid hex data"))
}
