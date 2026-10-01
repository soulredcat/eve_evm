// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{TxKind, U256};
use revm::context::TxEnv;

use super::infer_simulation_type::infer_simulation_type;
use super::{SimulationContext, SimulationError, SimulationRequest};

pub(super) fn build_simulation_transaction(
    context: &SimulationContext<'_>,
    request: &SimulationRequest,
) -> Result<TxEnv, SimulationError> {
    if request.data.len() > context.limits.maximum_calldata_bytes {
        return Err(SimulationError::Limit("simulation calldata"));
    }
    let gas = request
        .gas
        .unwrap_or(context.limits.maximum_gas.min(context.header.gas_limit));
    if gas > context.limits.maximum_gas || gas > context.header.gas_limit {
        return Err(SimulationError::Limit("simulation gas"));
    }
    let account = context.state.accounts.get(&request.from);
    if request.value > account.map_or(U256::ZERO, |account| account.balance) {
        return Err(SimulationError::InvalidRequest(
            "simulation value exceeds balance",
        ));
    }
    let transaction_type = infer_simulation_type(request)?;
    TxEnv::builder()
        .tx_type(Some(transaction_type))
        .caller(request.from)
        .kind(request.to.map_or(TxKind::Create, TxKind::Call))
        .value(request.value)
        .data(request.data.clone())
        .gas_limit(gas)
        .gas_price(if transaction_type == 2 {
            request.max_fee_per_gas.unwrap_or(0)
        } else {
            request.gas_price.unwrap_or(0)
        })
        .gas_priority_fee(
            (transaction_type == 2).then_some(request.max_priority_fee_per_gas.unwrap_or(0)),
        )
        .access_list(request.access_list.clone().unwrap_or_default())
        .nonce(account.map_or(0, |account| account.nonce))
        .chain_id(Some(context.state.identity.evm_chain_id))
        .build()
        .map_err(|_| SimulationError::InvalidRequest("simulation transaction fields"))
}
