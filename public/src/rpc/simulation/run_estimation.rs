// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_call_request::decode_call_request, reserve_simulation::reserve_simulation};
use crate::rpc::{
    RpcContext,
    encoding::{quantity, require_arity},
    errors::rpc_error,
    selectors::capture_selected_state,
};
use eve_evm::{SimulationContext, SimulationError, SimulationLimits, estimate_complete_state_gas};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn run_estimation(
    context: &RpcContext,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 1, 2)?;
    let request = decode_call_request(&params[0])?;
    let selected = capture_selected_state(
        context,
        params.get(1).unwrap_or(&Value::String("latest".into())),
    )?;
    let state = selected.commit();
    let (reservation, _permit) = reserve_simulation(context, state)?;
    let limits = SimulationLimits {
        maximum_gas: 30_000_000,
        maximum_calldata_bytes: 131_072,
        maximum_memory_bytes: context.simulation_memory_bytes,
        maximum_estimation_attempts: 32,
        maximum_access_list_entries: 256,
        maximum_access_list_storage_keys: 1024,
    };
    let simulation = SimulationContext {
        state: &state.state,
        version: &state.target,
        header: &state.block.header,
        state_budget: &context.state_budget,
        limits: &limits,
        reserved_clone_bytes: reservation,
    };
    match estimate_complete_state_gas(&simulation, &request) {
        Ok(result) => Ok(quantity(result.gas_limit)),
        Err(SimulationError::Revert(data)) => {
            Err(ErrorObjectOwned::owned(3, "execution reverted", Some(data)))
        }
        Err(error) => Err(rpc_error(-32000, format!("estimation rejected: {error:?}"))),
    }
}
