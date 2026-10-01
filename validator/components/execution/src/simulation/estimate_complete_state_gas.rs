// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    GasEstimate, SimulationContext, SimulationError, SimulationOutcome, SimulationRequest,
    simulate_complete_state,
};
use crate::{ExecutionResult, InvalidTransaction};

/// A bounded search with a successful final replay; gas-sensitive code need not be monotonic.
pub fn estimate_complete_state_gas(
    context: &SimulationContext<'_>,
    request: &SimulationRequest,
) -> Result<GasEstimate, SimulationError> {
    let mut trial = request.clone();
    let mut high = request
        .gas
        .unwrap_or(context.limits.maximum_gas.min(context.header.gas_limit));
    trial.gas = Some(high);
    let first = simulate_complete_state(context, &trial)?;
    match first.execution {
        ExecutionResult::Success { .. } => {}
        ExecutionResult::Revert { output, .. } => return Err(SimulationError::Revert(output)),
        ExecutionResult::Halt { reason, .. } => {
            return Err(SimulationError::Halt(format!("{reason:?}")));
        }
    }
    let mut low = 0_u64;
    let mut attempts = 1_usize;
    while high.saturating_sub(low) > 1 {
        if attempts + 1 >= context.limits.maximum_estimation_attempts {
            break;
        }
        let middle = low + (high - low) / 2;
        trial.gas = Some(middle);
        attempts += 1;
        match simulate_complete_state(context, &trial) {
            Ok(SimulationOutcome {
                execution: ExecutionResult::Success { .. },
            }) => high = middle,
            Ok(_)
            | Err(SimulationError::InvalidTransaction(
                InvalidTransaction::CallGasCostMoreThanGasLimit { .. }
                | InvalidTransaction::GasFloorMoreThanGasLimit { .. },
            )) => low = middle,
            Err(error) => return Err(error),
        }
    }
    trial.gas = Some(high);
    attempts += 1;
    let final_execution = simulate_complete_state(context, &trial)?.execution;
    let gas_used = final_execution.tx_gas_used();
    match final_execution {
        ExecutionResult::Success { .. } => Ok(GasEstimate {
            gas_limit: high,
            gas_used_at_limit: gas_used,
            attempts,
        }),
        ExecutionResult::Revert { output, .. } => Err(SimulationError::Revert(output)),
        ExecutionResult::Halt { reason, .. } => Err(SimulationError::Halt(format!("{reason:?}"))),
    }
}
