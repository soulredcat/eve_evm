// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::rpc_block_view::RpcBlockView;
use alloy_consensus::{Block, BlockBody, TxEnvelope};
use alloy_eips::eip2718::Decodable2718;
use anyhow::{Result, ensure};
pub(crate) fn measure_rpc_block_size(block: &RpcBlockView<'_>) -> Result<usize> {
    let mut transactions = Vec::new();
    for raw in &block.block.transactions {
        let mut remaining = raw.as_ref();
        let envelope = TxEnvelope::decode_2718(&mut remaining)?;
        ensure!(remaining.is_empty(), "trailing retained transaction bytes");
        transactions.push(envelope);
    }
    let canonical = Block {
        header: block.block.header.clone(),
        body: BlockBody {
            transactions,
            ommers: Vec::new(),
            withdrawals: Some(Default::default()),
        },
    };
    Ok(alloy_rlp::encode(&canonical).len())
}
