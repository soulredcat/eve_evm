// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::decode_hex::decode_hex;
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::types::{BlockId, PartSetHeader};
use serde_json::Value;
pub(super) fn decode_native_block_id(value: &Value) -> Result<BlockId> {
    ensure!(value.is_object(), "native block ID missing");
    let total = u32::try_from(
        value["parts"]["total"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("native part count missing"))?,
    )?;
    let hash = decode_hex(&value["hash"], 32, true)?;
    let part_hash = decode_hex(&value["parts"]["hash"], 32, true)?;
    ensure!(
        (total == 0 && hash.is_empty() && part_hash.is_empty())
            || (total > 0 && hash.len() == 32 && part_hash.len() == 32),
        "native block ID layout"
    );
    Ok(BlockId {
        hash,
        part_set_header: Some(PartSetHeader {
            total,
            hash: part_hash,
        }),
    })
}
