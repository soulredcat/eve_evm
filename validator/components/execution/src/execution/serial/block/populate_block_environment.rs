// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::BlockEnvironment;
use alloy_primitives::U256;
use revm::context::BlockEnv;

pub(crate) fn populate_block_environment(environment: &mut BlockEnv, block: &BlockEnvironment) {
    environment.number = U256::from(block.number);
    environment.timestamp = U256::from(block.timestamp);
    environment.gas_limit = block.gas_limit;
    environment.basefee = block.base_fee;
    environment.beneficiary = block.proposer;
    environment.difficulty = U256::ZERO;
    environment.prevrandao = Some(block.previous_consensus_hash);
    environment.blob_excess_gas_and_price = None;
    environment.slot_num = 0;
}
