// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::U256;
use revm::context::BlockEnv;

use super::SimulationContext;

pub(super) fn populate_simulation_block(block: &mut BlockEnv, context: &SimulationContext<'_>) {
    block.number = U256::from(context.header.number);
    block.timestamp = U256::from(context.header.timestamp);
    block.gas_limit = context.header.gas_limit;
    block.basefee = context.header.base_fee_per_gas.unwrap_or(0);
    block.beneficiary = context.header.beneficiary;
    block.prevrandao = Some(context.header.mix_hash);
}
