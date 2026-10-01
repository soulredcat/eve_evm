// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::Bytes;
use anyhow::Result;
use eve_evm::{
    BlockExecutionError, CompleteExecutionError, ExecutionBlockInput, execute_state_block,
};
use eve_state::{StateBudget, StateCommit};

/// Execute once, then at most once more for the already-valid prefix before an invalid item.
pub(super) fn execute_prepared_selection(
    parent: &StateCommit,
    input: &ExecutionBlockInput,
    mut transactions: Vec<Vec<u8>>,
    budget: &StateBudget,
    reserved_bytes: usize,
) -> Result<Vec<Vec<u8>>> {
    let payloads: Vec<Bytes> = transactions.iter().cloned().map(Bytes::from).collect();
    let first = execute_state_block(parent, input, &payloads, budget, reserved_bytes);
    match first {
        Ok(_) => Ok(transactions),
        Err(CompleteExecutionError::Execution(
            BlockExecutionError::InvalidTransaction { index, .. }
            | BlockExecutionError::BlockGasLimit { index },
        )) => {
            transactions.truncate(index);
            let prefix: Vec<Bytes> = transactions.iter().cloned().map(Bytes::from).collect();
            execute_state_block(parent, input, &prefix, budget, reserved_bytes)
                .map_err(|_| anyhow::anyhow!("prepared prefix unavailable"))?;
            Ok(transactions)
        }
        Err(_) => Err(anyhow::anyhow!("prepared execution unavailable")),
    }
}
