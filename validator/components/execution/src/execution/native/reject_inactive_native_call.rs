// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_protocol_config::native::{NATIVE_GAS_V1, SYSTEM_INTERFACE_ADDRESS};
use revm::{
    context_interface::ContextTr,
    handler::{EthPrecompiles, PrecompileProvider, precompile_output_to_interpreter_result},
    interpreter::{CallInputs, InterpreterResult},
    precompile::PrecompileOutput,
    primitives::Bytes,
};

/// Actual versioned revert bytes, not a success-shaped placeholder or ABI read result.
pub const NATIVE_INTERFACE_INACTIVE_REVERT_DATA: &[u8] = b"EVE_NATIVE_INTERFACE_INACTIVE_V1";

pub(crate) fn reject_inactive_native_call<CTX: ContextTr>(
    context: &mut CTX,
    inputs: &CallInputs,
    ethereum: &mut EthPrecompiles,
) -> Result<Option<InterpreterResult>, String> {
    if inputs.bytecode_address == SYSTEM_INTERFACE_ADDRESS {
        let output = PrecompileOutput::revert(
            NATIVE_GAS_V1.dispatch,
            Bytes::from_static(NATIVE_INTERFACE_INACTIVE_REVERT_DATA),
            inputs.reservoir,
        );
        return Ok(Some(precompile_output_to_interpreter_result(
            output,
            inputs.gas_limit,
        )));
    }
    ethereum.run(context, inputs)
}
