// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, B256};
use revm::context::TxEnv;

/// Signature-validated envelope; state/gas admission is still performed by REVM.
#[derive(Clone, Debug)]
pub struct ValidatedTransaction {
    pub(crate) hash: B256,
    pub(crate) sender: Address,
    pub(crate) transaction_type: u8,
    pub(crate) evm: TxEnv,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionValidationError {
    Empty,
    TooLarge,
    UnsupportedType,
    MalformedEncoding,
    NonCanonicalEncoding,
    WrongChain,
    InvalidSignature,
    InvalidFeeCaps,
    InvalidEnvironment,
}
