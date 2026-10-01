// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{BlockEnvironment, BlockExecutionError};

pub(crate) fn validate_block_environment(
    block: &BlockEnvironment,
) -> Result<(), BlockExecutionError> {
    if block.chain_id == 0 || block.gas_limit == 0 || block.base_fee == 0 {
        return Err(BlockExecutionError::InvalidEnvironment(
            "chain ID, gas limit, and base fee must be positive",
        ));
    }
    if block.maximum_transaction_bytes == 0 {
        return Err(BlockExecutionError::InvalidEnvironment(
            "transaction byte limit must be positive",
        ));
    }
    let pools = block.fee_pools;
    if pools.node_pool == pools.validator_pool
        || pools.node_pool == block.proposer
        || pools.validator_pool == block.proposer
    {
        return Err(BlockExecutionError::InvalidEnvironment(
            "fee escrows must be distinct from each other and the proposer",
        ));
    }
    Ok(())
}
