// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RpcContext, execute_rpc::execute_rpc, subscriptions::register_subscriptions};
use anyhow::Result;
use jsonrpsee::RpcModule;
use serde_json::Value;
use std::sync::Arc;
pub(crate) fn create_rpc_module(context: Arc<RpcContext>) -> Result<RpcModule<Arc<RpcContext>>> {
    let mut module = RpcModule::new(context);
    for method in [
        "web3_clientVersion",
        "net_version",
        "net_listening",
        "eth_chainId",
        "eth_syncing",
        "eth_blockNumber",
        "eth_getBalance",
        "eth_getCode",
        "eth_getStorageAt",
        "eth_getTransactionCount",
        "eth_sendRawTransaction",
        "eth_getTransactionByHash",
        "eth_getTransactionReceipt",
        "eth_getBlockByNumber",
        "eth_getBlockByHash",
        "eth_call",
        "eth_estimateGas",
        "eth_gasPrice",
        "eth_maxPriorityFeePerGas",
        "eth_feeHistory",
        "eth_getLogs",
        "eth_getProof",
        "eve_getFinalityProof",
        "eve_getNodeStatus",
        "eve_getStateRoots",
    ] {
        module.register_async_method(method, move |params, context, _| async move {
            let params = if params.len_bytes() == 0 {
                Vec::new()
            } else {
                params.parse::<Vec<Value>>()?
            };
            execute_rpc(Arc::clone(&context), method, params).await
        })?;
    }
    register_subscriptions(&mut module)?;
    Ok(module)
}
