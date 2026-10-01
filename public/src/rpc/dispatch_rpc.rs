// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    RpcContext,
    encoding::{quantity, require_arity},
    errors::rpc_error,
    history, reads, simulation,
};
use eve_protocol_config::headers::derive_next_base_fee;
use eve_storage::state::read_state_service;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::sync::atomic::Ordering;
pub(crate) fn dispatch_rpc(
    context: &RpcContext,
    method: &str,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    match method {
        "eth_getBalance" => reads::read_balance(context, params),
        "eth_getCode" => reads::read_code(context, params),
        "eth_getStorageAt" => reads::read_storage(context, params),
        "eth_getTransactionCount" => reads::read_nonce(context, params),
        "eth_getProof" => reads::read_proof(context, params),
        "eth_getBlockByNumber" => history::read_block(context, params, false),
        "eth_getBlockByHash" => history::read_block(context, params, true),
        "eth_getTransactionByHash" => history::read_transaction(context, params),
        "eth_getTransactionReceipt" => history::read_receipt(context, params),
        "eth_getLogs" => history::read_logs(context, params),
        "eth_feeHistory" => history::read_fee_history(context, params),
        "eth_call" => simulation::run_call(context, params),
        "eth_estimateGas" => simulation::run_estimation(context, params),
        "eve_getNodeStatus" => {
            require_arity(params, 0, 0)?;
            reads::read_node_status(context)
        }
        "eve_getFinalityProof" => Err(rpc_error(
            -32001,
            "FINALITY_UNAVAILABLE: local development has no authenticated validator finality",
        )),
        "web3_clientVersion" => {
            require_arity(params, 0, 0)?;
            Ok(Value::String(
                "EVE/development-0.1.0/Shanghai/LOCAL_DEV_UNAUTHENTICATED".into(),
            ))
        }
        "net_listening" => {
            require_arity(params, 0, 0)?;
            Ok(Value::Bool(context.healthy.load(Ordering::Acquire)))
        }
        "eth_syncing" => {
            require_arity(params, 0, 0)?;
            Ok(Value::Bool(false))
        }
        "net_version"
        | "eth_chainId"
        | "eth_blockNumber"
        | "eth_gasPrice"
        | "eth_maxPriorityFeePerGas" => {
            require_arity(params, 0, 0)?;
            let head = read_state_service(&context.service)
                .map_err(|e| rpc_error(-32000, e.to_string()))?;
            match method {
                "net_version" => Ok(Value::String(
                    head.commit().target.identity.evm_chain_id.to_string(),
                )),
                "eth_chainId" => Ok(quantity(head.commit().target.identity.evm_chain_id)),
                "eth_blockNumber" => Ok(quantity(head.commit().target.height)),
                "eth_maxPriorityFeePerGas" => Ok(quantity(1_u64)),
                _ => Ok(quantity(
                    derive_next_base_fee(&head.commit().block.header)
                        .map_err(|e| rpc_error(-32000, format!("base fee: {e:?}")))?
                        .checked_add(1)
                        .ok_or_else(|| rpc_error(-32000, "fee suggestion overflow"))?,
                )),
            }
        }
        _ => Err(rpc_error(-32601, "method not supported")),
    }
}
