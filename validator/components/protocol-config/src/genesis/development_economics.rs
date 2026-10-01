// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::U256;

use super::EconomicsParameters;

pub fn development_economics() -> EconomicsParameters {
    let unit = U256::from(1_000_000_000_000_000_000_u64);
    EconomicsParameters {
        burn_bps: 4_000,
        node_pool_bps: 3_000,
        validator_pool_bps: 3_000,
        epoch_blocks: 1_000,
        maximum_validators: 64,
        validator_self_bond: U256::from(10_000) * unit,
        node_self_bond: U256::from(100) * unit,
        default_commission_bps: 1_000,
        maximum_commission_bps: 2_000,
        unbonding_seconds: 604_800,
        unbonding_blocks: 2_000,
        maximum_tasks_per_block: 32,
        node_availability_bps: 8_000,
        double_sign_slash_bps: 500,
        downtime_window_blocks: 1_000,
        minimum_participation_bps: 8_000,
        node_service_slashing: false,
        default_issuance: U256::ZERO,
    }
}
