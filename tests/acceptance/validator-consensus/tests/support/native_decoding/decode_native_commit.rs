// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_base64::decode_base64, decode_decimal::decode_decimal, decode_hex::decode_hex,
    decode_native_block_id::decode_native_block_id, decode_timestamp::decode_timestamp,
};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::types::{Commit, CommitSig};
use serde_json::Value;
pub(crate) fn decode_native_commit(value: &Value) -> Result<Commit> {
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
        .map(|value| {
            let flag = i32::try_from(
                value["block_id_flag"]
                    .as_i64()
                    .ok_or_else(|| anyhow::anyhow!("native commit flag missing"))?,
            )?;
            ensure!(matches!(flag, 1..=3), "native commit flag range");
            let signature = decode_base64(
                value
                    .get("signature")
                    .ok_or_else(|| anyhow::anyhow!("native commit signature missing"))?,
                64,
                flag == 1,
            )?;
            let address = decode_hex(&value["validator_address"], 20, flag == 1)?;
            ensure!(
                (flag == 1 && signature.is_empty() && address.is_empty())
                    || (flag != 1 && signature.len() == 64 && address.len() == 20),
                "native commit signature layout"
            );
            Ok(CommitSig {
                block_id_flag: flag,
                validator_address: address,
                timestamp: Some(decode_timestamp(&value["timestamp"])?),
                signature,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Commit {
        height,
        round,
        block_id: Some(decode_native_block_id(&value["block_id"])?),
        signatures,
    })
}
