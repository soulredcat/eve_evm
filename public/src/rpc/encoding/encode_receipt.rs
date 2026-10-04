// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{encode_rpc_receipt::encode_rpc_receipt, rpc_block_view::RpcBlockView};
use anyhow::Result;
use eve_storage::state::RetainedBlockProjection;
use serde_json::Value;

pub(crate) fn encode_receipt(block: &RetainedBlockProjection, index: usize) -> Result<Value> {
    encode_rpc_receipt(
        &RpcBlockView {
            version: &block.version,
            block: &block.block,
        },
        index,
    )
}
