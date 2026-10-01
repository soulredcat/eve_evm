// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};

/// Recheck frozen native limits; validator admission/cryptographic genesis validation remains canonical.
pub(in crate::development::engine) fn validate_engine_public_genesis(
    bytes: &[u8],
) -> Result<serde_json::Value> {
    ensure!(bytes.len() <= 1_048_576, "native public genesis byte limit");
    let value: serde_json::Value = serde_json::from_slice(bytes)?;
    let limits = eve_protocol_config::genesis::development_consensus_limits();
    let parameters = &value["consensus_params"];
    ensure!(
        parameters["block"]["max_bytes"].as_str()
            == Some(limits.maximum_block_bytes.to_string().as_str())
            && parameters["block"]["max_gas"].as_str()
                == Some(limits.block_gas_limit.to_string().as_str()),
        "native genesis block/gas limits differ from frozen contract"
    );
    let age_nanos = limits
        .evidence_max_age_seconds
        .checked_mul(1_000_000_000)
        .ok_or_else(|| anyhow::anyhow!("native evidence duration overflow"))?;
    ensure!(
        parameters["evidence"]["max_age_num_blocks"].as_str()
            == Some(limits.evidence_max_age_blocks.to_string().as_str())
            && parameters["evidence"]["max_age_duration"].as_str()
                == Some(age_nanos.to_string().as_str())
            && parameters["evidence"]["max_bytes"].as_str() == Some("1048576"),
        "native genesis evidence limits differ from frozen contract"
    );
    ensure!(
        value["initial_height"].as_str() == Some("1")
            && value["chain_id"].as_str() == Some("eve-local-v1"),
        "native genesis development chain/height required"
    );
    let validators = value["validators"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("native genesis validator array missing"))?;
    let key_types = parameters["validator"]["pub_key_types"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("native genesis key types missing"))?;
    ensure!(
        validators.len() == 4
            && key_types.len() == 1
            && key_types[0].as_str() == Some("ed25519")
            && parameters["abci"]["vote_extensions_enable_height"].as_str() == Some("0"),
        "native genesis requires the frozen four-validator classical profile"
    );
    Ok(value)
}
