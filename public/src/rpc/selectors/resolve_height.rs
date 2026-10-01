// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{encoding::parse_quantity, errors::rpc_error};
use alloy_primitives::B256;
use eve_storage::state::{HistorySnapshot, lookup_execution_hash};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn resolve_height(
    snapshot: &HistorySnapshot<'_>,
    selector: &Value,
) -> Result<u64, ErrorObjectOwned> {
    match selector.as_str() {
        Some("latest") => Ok(snapshot.version().height),
        Some("pending") => Err(rpc_error(
            -32001,
            "PENDING_HISTORY_UNAVAILABLE: request applied block number or hash",
        )),
        Some("earliest") => Ok(0),
        Some("safe" | "finalized") => Err(rpc_error(
            -32001,
            "FINALITY_UNAVAILABLE: local development has no authenticated validator finality",
        )),
        Some(_) => u64::try_from(parse_quantity(selector)?)
            .map_err(|_| rpc_error(-32602, "block quantity overflow")),
        None => {
            let object = selector
                .as_object()
                .ok_or_else(|| rpc_error(-32602, "invalid block selector"))?;
            if object
                .keys()
                .any(|key| key != "blockHash" && key != "requireCanonical")
            {
                return Err(rpc_error(-32602, "unsupported selector field"));
            }
            if object
                .get("requireCanonical")
                .is_some_and(|value| !value.is_boolean())
            {
                return Err(rpc_error(-32602, "invalid requireCanonical"));
            }
            let hash = B256::from(crate::rpc::encoding::parse_fixed::<32>(
                object
                    .get("blockHash")
                    .ok_or_else(|| rpc_error(-32602, "selector requires blockHash"))?,
            )?);
            lookup_execution_hash(snapshot, hash)
                .map_err(|e| rpc_error(-32000, e.to_string()))?
                .ok_or_else(|| rpc_error(-32001, "UNKNOWN_BLOCK"))
        }
    }
}
