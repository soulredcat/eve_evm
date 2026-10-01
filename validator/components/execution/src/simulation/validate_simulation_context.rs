// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{compute_evm_root, validate_state_version};

use super::{SimulationContext, SimulationError};

pub(super) fn validate_simulation_context(
    context: &SimulationContext<'_>,
) -> Result<(), SimulationError> {
    validate_state_version(context.state, context.version, context.state_budget)
        .map_err(|error| SimulationError::Complete(crate::CompleteExecutionError::State(error)))?;
    if context.header.hash_slow() != context.version.execution_hash.0
        || context.header.number != context.version.height
        || context.header.timestamp != context.version.timestamp
        || context.header.state_root != compute_evm_root(&context.state.accounts).0
        || context.header.base_fee_per_gas.is_none_or(|fee| fee == 0)
        || context.limits.maximum_gas == 0
        || context.limits.maximum_memory_bytes == 0
        || context.limits.maximum_calldata_bytes == 0
        || context.limits.maximum_estimation_attempts < 2
        || context.limits.maximum_estimation_attempts > 64
    {
        return Err(SimulationError::InvalidRequest(
            "invalid captured environment or limits",
        ));
    }
    Ok(())
}
