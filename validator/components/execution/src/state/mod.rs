// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Complete logical-state bridge to the existing in-memory serial oracle.

mod blocks;
mod estimate_clone_reservation;
mod execute_complete_state;
mod from_revm_state;
mod project_revm_account;
mod project_revm_accounts;
mod to_revm_state;
mod types;
mod update_fee_ledger;

pub use blocks::{ExecutionBlockInput, PreparedStateBlock, execute_state_block};
pub use estimate_clone_reservation::estimate_clone_reservation;
pub use execute_complete_state::execute_complete_state;
pub use from_revm_state::from_revm_state;
pub(crate) use project_revm_accounts::project_revm_accounts;
pub use to_revm_state::to_revm_state;
pub use types::{CompleteBlockOutcome, CompleteExecutionError};
