// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use revm::{
    context::{CfgEnv, TxEnv},
    primitives::hardfork::SpecId,
};

use super::SimulationContext;

/// Simulation-only switches; canonical execution retains the default checks.
pub(super) fn populate_simulation_config(
    cfg: &mut CfgEnv,
    context: &SimulationContext<'_>,
    transaction: &TxEnv,
) {
    cfg.set_spec_and_mainnet_gas_params(SpecId::SHANGHAI);
    cfg.chain_id = context.state.identity.evm_chain_id;
    cfg.disable_nonce_check = true;
    cfg.disable_eip3607 = true;
    cfg.disable_base_fee = transaction.gas_price == 0;
    cfg.disable_fee_charge = true;
    cfg.memory_limit = context.limits.maximum_memory_bytes;
}
