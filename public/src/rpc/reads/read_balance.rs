// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{
    RpcContext,
    encoding::{parse_fixed, quantity, require_arity},
    selectors::capture_selected_state,
};
use alloy_primitives::{Address, U256};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn read_balance(
    context: &RpcContext,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 2, 2)?;
    let address = Address::from(parse_fixed::<20>(&params[0])?);
    let selected = capture_selected_state(context, &params[1])?;
    let state = selected.commit();
    Ok(quantity(
        state
            .state
            .accounts
            .get(&address)
            .map_or(U256::ZERO, |account| account.balance),
    ))
}
