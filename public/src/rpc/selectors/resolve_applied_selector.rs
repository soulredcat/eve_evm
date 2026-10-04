// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{
    encoding::{parse_fixed, parse_quantity},
    errors::rpc_error,
};
use crate::sync::applied::{AppliedPublication, applied_anchor, applied_commit};
use alloy_primitives::B256;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;

/// Only this captured RAM height is available; lack of an index never proves historical absence.
pub(crate) fn resolve_applied_selector(
    publication: &AppliedPublication,
    selector: &Value,
) -> Result<u64, ErrorObjectOwned> {
    let current = applied_commit(publication).target.height;
    let requested = match selector.as_str() {
        Some("latest") => current,
        Some("safe" | "finalized") => {
            if applied_anchor(publication).is_none() {
                return Err(rpc_error(
                    -32001,
                    "NOT_READY: local genesis has no certified application outcome",
                ));
            }
            current
        }
        Some("earliest") => 0,
        Some("pending") => {
            return Err(rpc_error(
                -32001,
                "NOT_READY: applied pending overlay unavailable",
            ));
        }
        Some(_) => u64::try_from(parse_quantity(selector)?)
            .map_err(|_| rpc_error(-32602, "block quantity overflow"))?,
        None => {
            let object = selector
                .as_object()
                .ok_or_else(|| rpc_error(-32602, "invalid block selector"))?;
            if object
                .keys()
                .any(|key| key != "blockHash" && key != "requireCanonical")
                || object
                    .get("requireCanonical")
                    .is_some_and(|value| !value.is_boolean())
            {
                return Err(rpc_error(-32602, "invalid applied block selector"));
            }
            let hash =
                B256::from(parse_fixed::<32>(object.get("blockHash").ok_or_else(
                    || rpc_error(-32602, "selector requires blockHash"),
                )?)?);
            if hash != applied_commit(publication).target.execution_hash.0 {
                return Err(rpc_error(
                    -32001,
                    "GAP: block hash is outside the captured applied view",
                ));
            }
            current
        }
    };
    if requested < current {
        return Err(rpc_error(
            -32001,
            "GAP: historical state is outside the captured applied view",
        ));
    }
    if requested > current {
        return Err(rpc_error(
            -32001,
            "NOT_READY: requested height is not locally applied",
        ));
    }
    Ok(requested)
}
