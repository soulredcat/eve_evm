// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Deterministic Shanghai execution and EVE fee accounting.
//!
//! The in-memory executor is a complete-state serial reference, not a durable
//! store, consensus implementation, or authenticated snapshot importer.

mod execution;
mod simulation;
mod state;
mod transactions;

pub use alloy_eips::eip2930::{AccessList, AccessListItem};

pub use state::{
    CompleteBlockOutcome, CompleteExecutionError, ExecutionBlockInput, PreparedStateBlock,
    estimate_clone_reservation, estimate_clone_reservation_ceiling, execute_complete_state,
    execute_state_block, from_revm_state, to_revm_state,
};

pub use execution::native::NATIVE_INTERFACE_INACTIVE_REVERT_DATA;
pub use execution::serial::block::{
    BlockEnvironment, BlockExecutionError, BlockOutcome, FeePoolAddresses, TransactionOutcome,
    execute_serial_block,
};
pub use execution::serial::fees::{FeeAllocation, split_collected_fees};
pub use execution::serial::roots::compute_state_root;
pub use revm::context_interface::result::{
    ExecutionResult, HaltReason, InvalidTransaction, Output,
};
pub use revm::database::InMemoryDB;
pub use simulation::{
    GasEstimate, SimulationContext, SimulationError, SimulationLimits, SimulationOutcome,
    SimulationRequest, estimate_complete_state_gas, simulate_complete_state,
};
pub use transactions::signed::{
    TransactionAdmission, TransactionAdmissionError, TransactionValidationError,
    ValidatedTransaction, check_transaction_admission, decode_signed_transaction,
};
