// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{encoding::parse_quantity, errors::rpc_error};
use alloy_primitives::{Address, TxKind, U256};
use alloy_rpc_types_eth::TransactionRequest;
use eve_evm::SimulationRequest;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn decode_call_request(value: &Value) -> Result<SimulationRequest, ErrorObjectOwned> {
    let object = value
        .as_object()
        .ok_or_else(|| rpc_error(-32602, "call request must be object"))?;
    let fields = [
        "from",
        "to",
        "value",
        "data",
        "input",
        "gas",
        "gasPrice",
        "maxFeePerGas",
        "maxPriorityFeePerGas",
        "type",
        "accessList",
    ];
    if object.keys().any(|key| !fields.contains(&key.as_str())) {
        return Err(rpc_error(-32602, "unsupported call request field"));
    }
    for key in [
        "value",
        "gas",
        "gasPrice",
        "maxFeePerGas",
        "maxPriorityFeePerGas",
        "type",
    ] {
        if let Some(value) = object.get(key) {
            parse_quantity(value)?;
        }
    }
    let request: TransactionRequest = {
        for field in ["data", "input"] {
            if let Some(value) = object.get(field) {
                crate::rpc::encoding::parse_data(value, 131_072)?;
            }
        }
        if let Some(access_list) = object.get("accessList") {
            super::validate_call_access_list::validate_call_access_list(access_list)?;
        }
        serde_json::from_value(value.clone()).map_err(|e| rpc_error(-32602, e.to_string()))?
    };
    let data = request
        .input
        .try_into_unique_input()
        .map_err(|_| rpc_error(-32602, "input and data differ"))?
        .unwrap_or_default();
    Ok(SimulationRequest {
        from: request.from.unwrap_or(Address::ZERO),
        to: request.to.and_then(|kind| match kind {
            TxKind::Call(address) => Some(address),
            TxKind::Create => None,
        }),
        value: request.value.unwrap_or(U256::ZERO),
        data,
        gas: request.gas,
        gas_price: request.gas_price,
        max_fee_per_gas: request.max_fee_per_gas,
        max_priority_fee_per_gas: request.max_priority_fee_per_gas,
        transaction_type: request.transaction_type,
        access_list: request.access_list,
    })
}
