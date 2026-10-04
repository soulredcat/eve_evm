// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_decimal::decode_decimal, decode_native_block_id::decode_native_block_id,
    decode_native_commit_signature::decode_native_commit_signature,
};
use crate::wire::tendermint::types::Commit;
use anyhow::{Result, ensure};
use serde_json::Value;
pub fn decode_native_commit(value: &Value) -> Result<Commit> {
    let signatures = value["signatures"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("native commit signatures missing"))?;
    ensure!(
        signatures.len() <= super::types::MAXIMUM_DECODED_VALIDATORS,
        "native commit signature count"
    );
    let height = decode_decimal(&value["height"])?;
    let round = i32::try_from(
        value["round"]
            .as_i64()
            .ok_or_else(|| anyhow::anyhow!("native commit round missing"))?,
    )?;
    ensure!(
        height >= 0 && round >= 0,
        "native commit height/round range"
    );
    let signatures = signatures
        .iter()
        .map(decode_native_commit_signature)
        .collect::<Result<Vec<_>>>()?;
    Ok(Commit {
        height,
        round,
        block_id: Some(decode_native_block_id(&value["block_id"])?),
        signatures,
    })
}
