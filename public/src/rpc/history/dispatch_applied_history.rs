// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::errors::rpc_error;
use crate::sync::applied::AppliedPublication;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;

/// These operations share the ordinary encoders and one charged immutable publication.
pub(crate) fn dispatch_applied_history(
    publication: &AppliedPublication,
    method: &str,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    match method {
        "eth_getBlockByNumber" => {
            super::read_applied_block::read_applied_block(publication, params, false)
        }
        "eth_getBlockByHash" => {
            super::read_applied_block::read_applied_block(publication, params, true)
        }
        "eth_getTransactionByHash" => {
            super::read_applied_transaction::read_applied_transaction(publication, params)
        }
        "eth_getTransactionReceipt" => {
            super::read_applied_receipt::read_applied_receipt(publication, params)
        }
        "eth_getLogs" => super::read_applied_logs::read_applied_logs(publication, params),
        "eth_feeHistory" => {
            super::read_applied_fee_history::read_applied_fee_history(publication, params)
        }
        _ => Err(rpc_error(-32601, "historical method not supported")),
    }
}
