// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CompleteBlockOutcome, CompleteExecutionError, from_revm_state, to_revm_state,
    update_fee_ledger::update_fee_ledger,
};
use crate::{BlockEnvironment, execute_serial_block};
use alloy_primitives::Bytes;
use eve_state::{CompleteState, StateBudget, StateError, StateVersion, project_state_journal};

pub fn execute_complete_state(
    parent: &CompleteState,
    version: &StateVersion,
    environment: &BlockEnvironment,
    transactions: &[Bytes],
    budget: &StateBudget,
    reserved_clone_bytes: usize,
) -> Result<CompleteBlockOutcome, CompleteExecutionError> {
    if version.height.checked_add(1) != Some(environment.number)
        || environment.timestamp < version.timestamp
        || environment.chain_id != parent.identity.evm_chain_id
    {
        return Err(CompleteExecutionError::State(StateError::ParentMismatch));
    }
    let cache = to_revm_state(parent, version, budget, reserved_clone_bytes)?;
    let execution = execute_serial_block(&cache, environment, transactions)
        .map_err(CompleteExecutionError::Execution)?;
    let mut state = from_revm_state(parent, &execution.state, budget)?;
    update_fee_ledger(&mut state, execution.fees)?;
    let journal = project_state_journal(parent, version, &state, environment.number, budget)
        .map_err(CompleteExecutionError::State)?;
    Ok(CompleteBlockOutcome {
        execution,
        state,
        journal,
    })
}
