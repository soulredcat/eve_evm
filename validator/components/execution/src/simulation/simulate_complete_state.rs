// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use core::{convert::Infallible, marker::PhantomData};
use revm::{
    Context, MainBuilder, MainContext, context_interface::result::EVMError, handler::Handler,
};

use super::{
    SimulationContext, SimulationError, SimulationOutcome, SimulationRequest,
    build_simulation_transaction::build_simulation_transaction,
    populate_simulation_block::populate_simulation_block,
    populate_simulation_config::populate_simulation_config,
    validate_simulation_context::validate_simulation_context,
    validate_simulation_request_bounds::validate_simulation_request_bounds,
};
use crate::{
    execution::{native::inactive_native_provider, serial::fees::handler::EveFeeHandler},
    to_revm_state,
};

/// Run unsigned execution on a private oracle and discard every state/fee effect.
pub fn simulate_complete_state(
    context: &SimulationContext<'_>,
    request: &SimulationRequest,
) -> Result<SimulationOutcome, SimulationError> {
    validate_simulation_request_bounds(request, context.limits)?;
    validate_simulation_context(context)?;
    let transaction = build_simulation_transaction(context, request)?;
    let database = to_revm_state(
        context.state,
        context.version,
        context.state_budget,
        context.reserved_clone_bytes,
    )
    .map_err(SimulationError::Complete)?;
    let execution_context = Context::mainnet()
        .modify_cfg_chained(|cfg| populate_simulation_config(cfg, context, &transaction))
        .modify_block_chained(|block| populate_simulation_block(block, context))
        .with_db(database);
    let mut evm = execution_context
        .build_mainnet()
        .with_precompiles(inactive_native_provider());
    evm.ctx.tx = transaction;
    let mut handler: EveFeeHandler<_, EVMError<Infallible>, _> = EveFeeHandler {
        marker: PhantomData,
    };
    let execution = handler.run(&mut evm).map_err(|error| match error {
        EVMError::Transaction(error) => SimulationError::InvalidTransaction(error),
        other => SimulationError::Execution(format!("{other:?}")),
    })?;
    Ok(SimulationOutcome { execution })
}
