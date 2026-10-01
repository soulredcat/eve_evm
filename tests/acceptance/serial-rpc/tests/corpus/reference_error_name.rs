// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use revm::context_interface::result::InvalidTransaction;

pub fn reference_error_name(error: InvalidTransaction) -> &'static str {
    match error {
        InvalidTransaction::CallGasCostMoreThanGasLimit { .. } => {
            "TransactionException.INTRINSIC_GAS_TOO_LOW"
        }
        InvalidTransaction::LackOfFundForMaxFee { .. } => {
            "TransactionException.INSUFFICIENT_ACCOUNT_FUNDS"
        }
        InvalidTransaction::NonceOverflowInTransaction => "TransactionException.NONCE_IS_MAX",
        InvalidTransaction::NonceTooLow { .. } => "TransactionException.NONCE_MISMATCH_TOO_LOW",
        InvalidTransaction::NonceTooHigh { .. } => "TransactionException.NONCE_MISMATCH_TOO_HIGH",
        InvalidTransaction::InvalidChainId => "TransactionException.INVALID_CHAINID",
        InvalidTransaction::CreateInitCodeSizeLimit => {
            "TransactionException.INITCODE_SIZE_EXCEEDED"
        }
        other => panic!("Unexpected reference-validation category: {other:?}"),
    }
}
