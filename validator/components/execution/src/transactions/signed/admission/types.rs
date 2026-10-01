// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, B256, U256};

/// State-derived admission data; future/stale nonce policy belongs to public ingress.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionAdmission {
    pub hash: B256,
    pub sender: Address,
    pub nonce: u64,
    pub state_nonce: u64,
    pub transaction_type: u8,
    pub gas_limit: u64,
    pub intrinsic_gas: u64,
    pub max_fee_per_gas: u128,
    pub max_priority_fee_per_gas: Option<u128>,
    pub effective_gas_price: u128,
    pub value: U256,
    pub maximum_upfront_cost: U256,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransactionAdmissionError {
    InvalidEnvironment,
    IntrinsicGas,
    CallerHasCode,
    FeeCaps,
    GasLimit,
    InsufficientBalance,
    ArithmeticOverflow,
}
