// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{encoding::parse_fixed, errors::rpc_error};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn validate_call_access_list(value: &Value) -> Result<(), ErrorObjectOwned> {
    let entries = value
        .as_array()
        .ok_or_else(|| rpc_error(-32602, "accessList must be array"))?;
    if entries.len() > 256 {
        return Err(rpc_error(-32602, "accessList entry cap256 exceeded"));
    }
    let mut slots = 0_usize;
    for entry in entries {
        let item = entry
            .as_object()
            .ok_or_else(|| rpc_error(-32602, "accessList item must be object"))?;
        if item
            .keys()
            .any(|field| field != "address" && field != "storageKeys")
        {
            return Err(rpc_error(-32602, "unknown accessList item field"));
        }
        parse_fixed::<20>(
            item.get("address")
                .ok_or_else(|| rpc_error(-32602, "accessList address missing"))?,
        )?;
        let keys = item
            .get("storageKeys")
            .and_then(Value::as_array)
            .ok_or_else(|| rpc_error(-32602, "accessList storageKeys array missing"))?;
        slots = slots
            .checked_add(keys.len())
            .ok_or_else(|| rpc_error(-32602, "accessList key accounting overflow"))?;
        if slots > 1024 {
            return Err(rpc_error(-32602, "accessList storage key cap1024 exceeded"));
        }
        for key in keys {
            parse_fixed::<32>(key)?;
        }
    }
    Ok(())
}
