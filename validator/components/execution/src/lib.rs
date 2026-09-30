//! Deterministic Shanghai execution and EVE fee accounting.
//!
//! The in-memory executor is a complete-state serial reference, not a durable
//! store, consensus implementation, or authenticated snapshot importer.

mod execution;
mod state;
mod transactions;

pub use state::{
    CompleteBlockOutcome, CompleteExecutionError, estimate_clone_reservation,
    execute_complete_state, from_revm_state, to_revm_state,
};

pub use execution::serial::block::{
    BlockEnvironment, BlockExecutionError, BlockOutcome, FeePoolAddresses, TransactionOutcome,
    execute_serial_block,
};
pub use execution::serial::fees::{FeeAllocation, split_collected_fees};
pub use execution::serial::roots::compute_state_root;
pub use revm::database::InMemoryDB;
pub use transactions::signed::{TransactionValidationError, decode_signed_transaction};
