// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::LogFilter;
use crate::rpc::{encoding::parse_fixed, errors::rpc_error};
use alloy_primitives::{Address, B256};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::collections::BTreeSet;
pub(crate) fn decode_log_filter(value: &Value) -> Result<LogFilter, ErrorObjectOwned> {
    let object = value
        .as_object()
        .ok_or_else(|| rpc_error(-32602, "log filter must be object"))?;
    if object.keys().any(|key| {
        !["address", "topics", "fromBlock", "toBlock", "blockHash"].contains(&key.as_str())
    }) {
        return Err(rpc_error(-32602, "unsupported log filter field"));
    }
    let addresses = if let Some(value) = object.get("address") {
        let items = value
            .as_array()
            .cloned()
            .unwrap_or_else(|| vec![value.clone()]);
        if items.len() > 64 {
            return Err(rpc_error(-32602, "log address cap64 exceeded"));
        }
        Some(
            items
                .iter()
                .map(|item| parse_fixed::<20>(item).map(Address::from))
                .collect::<Result<BTreeSet<_>, _>>()?,
        )
    } else {
        None
    };
    let mut topics = Vec::new();
    if let Some(value) = object.get("topics") {
        let items = value
            .as_array()
            .ok_or_else(|| rpc_error(-32602, "topics must be array"))?;
        if items.len() > 4 {
            return Err(rpc_error(-32602, "topic positions exceed4"));
        }
        for item in items {
            if item.is_null() {
                topics.push(None);
                continue;
            }
            let alternatives = item
                .as_array()
                .cloned()
                .unwrap_or_else(|| vec![item.clone()]);
            if alternatives.len() > 64 {
                return Err(rpc_error(-32602, "topic alternatives cap64 exceeded"));
            }
            if alternatives.iter().any(Value::is_null) {
                topics.push(None);
                continue;
            }
            topics.push(Some(
                alternatives
                    .iter()
                    .map(|value| parse_fixed::<32>(value).map(B256::from))
                    .collect::<Result<BTreeSet<_>, _>>()?,
            ));
        }
    }
    let block_hash = object
        .get("blockHash")
        .map(|value| parse_fixed::<32>(value).map(B256::from))
        .transpose()?;
    if block_hash.is_some() && (object.contains_key("fromBlock") || object.contains_key("toBlock"))
    {
        return Err(rpc_error(-32602, "blockHash conflicts with block range"));
    }
    Ok(LogFilter {
        addresses,
        topics,
        from: object.get("fromBlock").cloned(),
        to: object.get("toBlock").cloned(),
        block_hash,
    })
}
