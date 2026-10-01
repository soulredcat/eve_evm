// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{build_development_consensus_params, native_genesis_types::*};
use anyhow::{Result, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use eve_consensus_comet::consensus::certificates::{
    ClassicalValidator, canonicalize_validator_set, validator_address,
};
use eve_protocol_config::genesis::DevelopmentGenesis;
use eve_state::StateCommit;

pub(crate) fn build_native_genesis_document(
    genesis: &DevelopmentGenesis,
    initial: &StateCommit,
    app_state: serde_json::Value,
) -> Result<Vec<u8>> {
    ensure!(
        initial.target.height == 0 && initial.target.application.is_none(),
        "native genesis requires the canonical height-zero state"
    );
    let time = prost_types::Timestamp {
        seconds: i64::try_from(genesis.initial_timestamp)?,
        nanos: 0,
    };
    let members = genesis
        .validators
        .iter()
        .map(|validator| {
            Ok(ClassicalValidator {
                public_key: validator.classical_public_key,
                voting_power: i64::try_from(validator.voting_power)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let members = canonicalize_validator_set(&members)
        .map_err(|_| anyhow::anyhow!("invalid native genesis validator set"))?;
    let native = build_development_consensus_params(genesis.protocol_version);
    let block = native
        .block
        .ok_or_else(|| anyhow::anyhow!("block parameters missing"))?;
    let evidence = native
        .evidence
        .ok_or_else(|| anyhow::anyhow!("evidence parameters missing"))?;
    let age_seconds = evidence
        .max_age_duration
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("evidence duration missing"))?
        .seconds;
    let age_nanos = age_seconds
        .checked_mul(1_000_000_000)
        .ok_or_else(|| anyhow::anyhow!("evidence duration overflow"))?;
    let document = NativeGenesisDocument {
        genesis_time: time.to_string(),
        chain_id: genesis.network_name.clone(),
        initial_height: "1".into(),
        consensus_params: NativeGenesisParameters {
            block: NativeBlockParameters {
                max_bytes: block.max_bytes.to_string(),
                max_gas: block.max_gas.to_string(),
            },
            evidence: NativeEvidenceParameters {
                max_age_num_blocks: evidence.max_age_num_blocks.to_string(),
                max_age_duration: age_nanos.to_string(),
                max_bytes: evidence.max_bytes.to_string(),
            },
            validator: NativeValidatorParameters {
                pub_key_types: vec!["ed25519".into()],
            },
            version: NativeVersionParameters {
                app: u64::from(genesis.protocol_version).to_string(),
            },
            abci: NativeAbciParameters {
                vote_extensions_enable_height: "0".into(),
            },
            authority: NativeAuthorityParameters {
                authority: String::new(),
            },
        },
        validators: members
            .iter()
            .enumerate()
            .map(|(index, validator)| NativeGenesisValidator {
                address: hex::encode_upper(validator_address(&validator.public_key)),
                pub_key: NativeGenesisPublicKey {
                    key_type: "tendermint/PubKeyEd25519".into(),
                    value: STANDARD.encode(validator.public_key),
                },
                power: validator.voting_power.to_string(),
                name: format!("eve-development-validator-{index}"),
            })
            .collect(),
        app_hash: hex::encode_upper(initial.target.content_digest),
        app_state,
    };
    let bytes = serde_json::to_vec_pretty(&document)?;
    ensure!(bytes.len() <= 1_048_576, "native genesis byte limit");
    Ok(bytes)
}
