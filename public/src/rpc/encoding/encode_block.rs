// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{encode_rpc_block::encode_rpc_block, rpc_block_view::RpcBlockView};
use anyhow::Result;
use eve_storage::state::RetainedBlockProjection;
use serde_json::Value;

pub(crate) fn encode_block(block: &RetainedBlockProjection, full: bool) -> Result<Value> {
    encode_rpc_block(
        &RpcBlockView {
            version: &block.version,
            block: &block.block,
        },
        full,
    )
}
