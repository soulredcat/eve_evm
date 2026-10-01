// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use revm::{
    context_interface::{Cfg, ContextTr},
    handler::{EthPrecompiles, PrecompileProvider},
    interpreter::{CallInputs, InterpreterResult},
    primitives::AddressSet,
};

use super::{
    native_warm_addresses::native_warm_addresses,
    refresh_native_precompiles::refresh_native_precompiles,
    reject_inactive_native_call::reject_inactive_native_call,
};

pub(crate) struct InactiveNativeProvider {
    pub(crate) ethereum: EthPrecompiles,
    pub(crate) warmed: AddressSet,
}

/// Thin upstream-trait delegation; dispatch and refresh own their operations.
impl<CTX: ContextTr> PrecompileProvider<CTX> for InactiveNativeProvider {
    type Output = InterpreterResult;

    fn set_spec(&mut self, spec: <CTX::Cfg as Cfg>::Spec) -> bool {
        refresh_native_precompiles(self, spec)
    }

    fn run(
        &mut self,
        context: &mut CTX,
        inputs: &CallInputs,
    ) -> Result<Option<InterpreterResult>, String> {
        reject_inactive_native_call(context, inputs, &mut self.ethereum)
    }

    fn warm_addresses(&self) -> &AddressSet {
        native_warm_addresses(self)
    }
}
