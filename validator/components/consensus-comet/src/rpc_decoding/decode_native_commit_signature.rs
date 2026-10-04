// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_base64::decode_base64, decode_hex::decode_hex, decode_timestamp::decode_timestamp,
};
use crate::wire::tendermint::types::CommitSig;
use anyhow::{Result, ensure};
use serde_json::Value;

pub(super) fn decode_native_commit_signature(value: &Value) -> Result<CommitSig> {
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
}
