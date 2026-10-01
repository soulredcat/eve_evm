// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::errors::rpc_error;
use jsonrpsee::types::ErrorObjectOwned;
pub(crate) fn estimate_json_string_bytes(
    value: &str,
    maximum: usize,
) -> Result<usize, ErrorObjectOwned> {
    let mut bytes = 2_usize;
    for character in value.bytes() {
        let charge = match character {
            b'"' | b'\\' | b'\n' | b'\r' | b'\t' | 8 | 12 => 2,
            0..=31 => 6,
            _ => 1,
        };
        bytes = bytes
            .checked_add(charge)
            .ok_or_else(|| rpc_error(-32005, "JSON string accounting overflow"))?;
        if bytes > maximum {
            return Err(rpc_error(-32005, "JSON output byte capacity exceeded"));
        }
    }
    Ok(bytes)
}
