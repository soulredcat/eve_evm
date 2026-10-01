// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::errors::rpc_error;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
/// Logical request/result serialization reservation, independent of state/VM leases.
pub(crate) fn estimate_rpc_reservation(
    method: &str,
    params: &[Value],
) -> Result<u32, ErrorObjectOwned> {
    let request = serde_json::to_vec(params)
        .map_err(|e| rpc_error(-32603, e.to_string()))?
        .len();
    let response: usize = match method {
        "eth_getBalance"
        | "eth_getStorageAt"
        | "eth_getTransactionCount"
        | "eth_chainId"
        | "eth_blockNumber"
        | "net_version"
        | "net_listening"
        | "eth_syncing"
        | "eth_gasPrice"
        | "eth_maxPriorityFeePerGas"
        | "web3_clientVersion"
        | "eve_getFinalityProof"
        | "eve_getNodeStatus" => 4096,
        "eth_getCode" => 65_536,
        _ => 4 * 1_048_576,
    };
    let bytes = request
        .checked_mul(3)
        .and_then(|bytes| {
            response
                .checked_mul(3)
                .and_then(|response| bytes.checked_add(response))
        })
        .ok_or_else(|| rpc_error(-32005, "RPC buffer accounting overflow"))?;
    u32::try_from(bytes.div_ceil(1024))
        .map_err(|_| rpc_error(-32005, "RPC buffer reservation overflow"))
}
