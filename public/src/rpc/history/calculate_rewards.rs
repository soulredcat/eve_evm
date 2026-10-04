// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::calculate_rpc_rewards::calculate_rpc_rewards;
use crate::rpc::encoding::RpcBlockView;
use anyhow::Result;
use eve_storage::state::RetainedBlockProjection;

pub(crate) fn calculate_rewards(
    block: &RetainedBlockProjection,
    percentiles: &[f64],
) -> Result<Vec<u128>> {
    calculate_rpc_rewards(
        &RpcBlockView {
            version: &block.version,
            block: &block.block,
        },
        percentiles,
    )
}
