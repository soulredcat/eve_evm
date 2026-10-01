// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::{Eip658Value, Receipt, ReceiptEnvelope};
use revm::context_interface::result::ExecutionResult;

pub(crate) fn build_receipt(
    transaction_type: u8,
    execution: &ExecutionResult,
    cumulative_gas_used: u64,
) -> ReceiptEnvelope {
    let receipt = Receipt {
        status: Eip658Value::Eip658(execution.is_success()),
        cumulative_gas_used,
        logs: execution.logs().to_vec(),
    }
    .with_bloom();
    match transaction_type {
        0 => ReceiptEnvelope::Legacy(receipt),
        1 => ReceiptEnvelope::Eip2930(receipt),
        2 => ReceiptEnvelope::Eip1559(receipt),
        _ => unreachable!("only validated Shanghai transaction types reach receipt construction"),
    }
}
