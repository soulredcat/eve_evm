// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, Bytes, U256};
use eve_state::{CompleteState, Header, StateBudget, StateVersion};

use crate::{CompleteExecutionError, ExecutionResult};
use alloy_eips::eip2930::AccessList;
use revm::context_interface::result::InvalidTransaction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimulationRequest {
    pub from: Address,
    pub to: Option<Address>,
    pub value: U256,
    pub data: Bytes,
    pub gas: Option<u64>,
    pub gas_price: Option<u128>,
    pub max_fee_per_gas: Option<u128>,
    pub max_priority_fee_per_gas: Option<u128>,
    pub transaction_type: Option<u8>,
    pub access_list: Option<AccessList>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimulationLimits {
    pub maximum_gas: u64,
    pub maximum_calldata_bytes: usize,
    pub maximum_memory_bytes: u64,
    pub maximum_estimation_attempts: usize,
    pub maximum_access_list_entries: usize,
    pub maximum_access_list_storage_keys: usize,
}

#[derive(Clone, Copy)]
pub struct SimulationContext<'a> {
    pub state: &'a CompleteState,
    pub version: &'a StateVersion,
    pub header: &'a Header,
    pub state_budget: &'a StateBudget,
    pub limits: &'a SimulationLimits,
    pub reserved_clone_bytes: usize,
}

#[derive(Debug)]
pub struct SimulationOutcome {
    pub execution: ExecutionResult,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GasEstimate {
    pub gas_limit: u64,
    pub gas_used_at_limit: u64,
    pub attempts: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimulationError {
    Complete(CompleteExecutionError),
    InvalidRequest(&'static str),
    Limit(&'static str),
    Execution(String),
    InvalidTransaction(InvalidTransaction),
    Revert(Bytes),
    Halt(String),
}
