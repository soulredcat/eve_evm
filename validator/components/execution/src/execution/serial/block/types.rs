// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::execution::serial::fees::FeeAllocation;
use crate::transactions::signed::TransactionValidationError;
use alloy_consensus::ReceiptEnvelope;
use alloy_primitives::{Address, B256, Bytes};
use revm::{context_interface::result::ExecutionResult, database::InMemoryDB};

/// Distinct development escrows; addresses are explicit consensus inputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeePoolAddresses {
    pub node_pool: Address,
    pub validator_pool: Address,
}

/// Agreed block inputs; no field is derived from the executor's host clock.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockEnvironment {
    pub chain_id: u64,
    pub number: u64,
    pub timestamp: u64,
    pub gas_limit: u64,
    pub base_fee: u64,
    pub proposer: Address,
    pub previous_consensus_hash: B256,
    pub fee_pools: FeePoolAddresses,
    pub maximum_transaction_bytes: usize,
}

#[derive(Debug)]
pub struct TransactionOutcome {
    pub hash: B256,
    pub sender: Address,
    pub effective_gas_price: u128,
    pub execution: ExecutionResult,
}

/// Atomic candidate result. The caller publishes it only after consensus.
#[derive(Debug)]
pub struct BlockOutcome {
    pub state: InMemoryDB,
    pub transactions: Vec<Bytes>,
    pub outcomes: Vec<TransactionOutcome>,
    pub receipts: Vec<ReceiptEnvelope>,
    pub gas_used: u64,
    pub fees: FeeAllocation,
    pub state_root: B256,
    pub transactions_root: B256,
    pub receipts_root: B256,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BlockExecutionError {
    InvalidEnvironment(&'static str),
    InvalidTransaction {
        index: usize,
        reason: TransactionValidationError,
    },
    Execution {
        index: usize,
        message: String,
    },
    BlockGasLimit {
        index: usize,
    },
    ArithmeticOverflow,
}
