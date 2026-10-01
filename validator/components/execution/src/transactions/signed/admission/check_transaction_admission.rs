// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{KECCAK256_EMPTY, U256};
use eve_state::StateAccount;
use revm::{
    context_interface::Transaction, handler::validation::validate_initial_tx_gas,
    primitives::hardfork::SpecId,
};

use super::{TransactionAdmission, TransactionAdmissionError};
use crate::ValidatedTransaction;

/// Check one captured sender account without selecting mempool nonce policy.
pub fn check_transaction_admission(
    transaction: &ValidatedTransaction,
    account: Option<&StateAccount>,
    base_fee: u64,
    block_gas_limit: u64,
) -> Result<TransactionAdmission, TransactionAdmissionError> {
    if base_fee == 0 || block_gas_limit == 0 {
        return Err(TransactionAdmissionError::InvalidEnvironment);
    }
    let tx = &transaction.evm;
    if tx.gas_limit > block_gas_limit {
        return Err(TransactionAdmissionError::GasLimit);
    }
    if account.is_some_and(|account| account.code_hash != KECCAK256_EMPTY) {
        return Err(TransactionAdmissionError::CallerHasCode);
    }
    if tx.gas_price < u128::from(base_fee)
        || tx.gas_priority_fee.is_some_and(|tip| tip > tx.gas_price)
    {
        return Err(TransactionAdmissionError::FeeCaps);
    }
    let intrinsic = validate_initial_tx_gas(tx, SpecId::SHANGHAI, false, false, u64::MAX, None)
        .map_err(|_| TransactionAdmissionError::IntrinsicGas)?;
    let maximum_upfront_cost = U256::from(tx.gas_price)
        .checked_mul(U256::from(tx.gas_limit))
        .and_then(|fee| fee.checked_add(tx.value))
        .ok_or(TransactionAdmissionError::ArithmeticOverflow)?;
    if maximum_upfront_cost > account.map_or(U256::ZERO, |account| account.balance) {
        return Err(TransactionAdmissionError::InsufficientBalance);
    }
    Ok(TransactionAdmission {
        hash: transaction.hash,
        sender: transaction.sender,
        nonce: tx.nonce,
        state_nonce: account.map_or(0, |account| account.nonce),
        transaction_type: transaction.transaction_type,
        gas_limit: tx.gas_limit,
        intrinsic_gas: intrinsic.initial_total_gas(),
        max_fee_per_gas: tx.gas_price,
        max_priority_fee_per_gas: tx.gas_priority_fee,
        effective_gas_price: tx.effective_gas_price(u128::from(base_fee)),
        value: tx.value,
        maximum_upfront_cost,
    })
}
