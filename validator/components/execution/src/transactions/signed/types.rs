use alloy_primitives::{Address, B256};
use revm::context::TxEnv;

/// Signature-validated envelope; state/gas admission is still performed by REVM.
#[derive(Clone, Debug)]
pub struct ValidatedTransaction {
    pub hash: B256,
    pub sender: Address,
    pub transaction_type: u8,
    pub evm: TxEnv,
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
