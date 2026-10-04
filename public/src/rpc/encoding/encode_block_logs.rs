// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{encode_rpc_block_logs::encode_rpc_block_logs, rpc_block_view::RpcBlockView};
use anyhow::Result;
use eve_storage::state::RetainedBlockProjection;
use serde_json::Value;

pub(crate) fn encode_block_logs(block: &RetainedBlockProjection) -> Result<Vec<Value>> {
    encode_rpc_block_logs(&RpcBlockView {
        version: &block.version,
        block: &block.block,
    })
}
