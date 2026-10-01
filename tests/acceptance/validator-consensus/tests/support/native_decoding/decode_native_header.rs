// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_decimal::decode_decimal, decode_hex::decode_hex,
    decode_native_block_id::decode_native_block_id, decode_timestamp::decode_timestamp,
};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::{types::Header, version::Consensus};
use serde_json::Value;
pub(crate) fn decode_native_header(value: &Value) -> Result<Header> {
    let chain_id = value["chain_id"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("native chain ID missing"))?;
    ensure!(
        !chain_id.is_empty() && chain_id.len() <= 50,
        "native chain ID bound"
    );
    let height = decode_decimal(&value["height"])?;
    ensure!(height > 0, "native header height");
    Ok(Header {
        version: Some(Consensus {
            block: decode_decimal(&value["version"]["block"])?,
            app: decode_decimal(&value["version"]["app"])?,
        }),
        chain_id: chain_id.into(),
        height,
        time: Some(decode_timestamp(&value["time"])?),
        last_block_id: Some(decode_native_block_id(&value["last_block_id"])?),
        last_commit_hash: decode_hex(&value["last_commit_hash"], 32, true)?,
        data_hash: decode_hex(&value["data_hash"], 32, true)?,
        validators_hash: decode_hex(&value["validators_hash"], 32, false)?,
        next_validators_hash: decode_hex(&value["next_validators_hash"], 32, false)?,
        consensus_hash: decode_hex(&value["consensus_hash"], 32, false)?,
        app_hash: decode_hex(&value["app_hash"], 32, true)?,
        last_results_hash: decode_hex(&value["last_results_hash"], 32, true)?,
        evidence_hash: decode_hex(&value["evidence_hash"], 32, true)?,
        proposer_address: decode_hex(&value["proposer_address"], 20, false)?,
    })
}
