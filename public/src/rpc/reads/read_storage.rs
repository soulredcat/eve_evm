// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{
    RpcContext,
    encoding::{parse_fixed, parse_storage_slot, require_arity},
    selectors::capture_selected_state,
};
use alloy_primitives::{Address, B256, U256};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn read_storage(
    context: &RpcContext,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 3, 3)?;
    let address = Address::from(parse_fixed::<20>(&params[0])?);
    let slot = parse_storage_slot(&params[1])?;
    let selected = capture_selected_state(context, &params[2])?;
    let state = selected.commit();
    let value = state
        .state
        .accounts
        .get(&address)
        .and_then(|account| account.storage.get(&slot))
        .copied()
        .unwrap_or(U256::ZERO);
    serde_json::to_value(B256::from(value.to_be_bytes::<32>()))
        .map_err(|e| crate::rpc::errors::rpc_error(-32603, e.to_string()))
}
