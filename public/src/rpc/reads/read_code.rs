// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{
    RpcContext,
    encoding::{parse_fixed, require_arity},
    errors::rpc_error,
    selectors::capture_selected_state,
};
use alloy_primitives::{Address, Bytes};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn read_code(context: &RpcContext, params: &[Value]) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 2, 2)?;
    let address = Address::from(parse_fixed::<20>(&params[0])?);
    let selected = capture_selected_state(context, &params[1])?;
    let state = selected.commit();
    let code = state
        .state
        .accounts
        .get(&address)
        .and_then(|account| state.state.codes.get(&account.code_hash))
        .cloned()
        .unwrap_or_else(Bytes::new);
    serde_json::to_value(code).map_err(|e| rpc_error(-32603, e.to_string()))
}
