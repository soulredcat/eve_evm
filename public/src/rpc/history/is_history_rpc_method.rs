// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(crate) fn is_history_rpc_method(method: &str) -> bool {
    matches!(
        method,
        "eth_getBlockByNumber"
            | "eth_getBlockByHash"
            | "eth_getTransactionByHash"
            | "eth_getTransactionReceipt"
            | "eth_getLogs"
            | "eth_feeHistory"
    )
}
