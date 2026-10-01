// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::build_fixture_configuration::build_fixture_configuration;
use crate::support::{
    ClusterOptions, cluster::Node, compile_transition_contract::compile_transition_contract,
};
use alloy_primitives::{Address, keccak256};
use anyhow::{Context, Result};
use eve_protocol_config::genesis::input::decode_development_spec;
use eve_state::{StateCommit, development_state_budget, initialize_development_state};
use k256::{ecdsa::SigningKey, elliptic_curve::rand_core::OsRng};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

type GenesisMaterial = (
    PathBuf,
    StateCommit,
    SigningKey,
    Address,
    Option<PathBuf>,
    [u8; 32],
);

pub(crate) fn write_cluster_genesis(
    artifact: &Path,
    nodes: &[Node],
    options: &ClusterOptions,
) -> Result<GenesisMaterial> {
    let authority = SigningKey::random(&mut OsRng);
    let encoded = authority.verifying_key().to_encoded_point(false);
    let authority_address =
        Address::from_slice(&keccak256(&encoded.as_bytes()[1..]).as_slice()[12..]);
    let mut accounts: Vec<_> = nodes
        .iter()
        .take(4)
        .map(|node| {
            serde_json::json!({
                "address": node.owner,
                "funded_balance": "1000000000000000000000000",
                "nonce": 0,
                "code": "0x"
            })
        })
        .collect();
    accounts.push(serde_json::json!({
        "address": authority_address,
        "funded_balance": "1000000000000000000000000",
        "nonce": 0,
        "code": "0x"
    }));
    accounts.push(serde_json::json!({
        "address": format!("0x{}", super::REVERT_ADDRESS),
        "funded_balance": "0",
        "nonce": 0,
        "code": "0x60006000fd"
    }));
    let mut fixture_path = None;
    let mut digest = [0; 32];
    if options.transitions || options.poison {
        let fixture = build_fixture_configuration(nodes, options, authority_address)?;
        let bytes = serde_json::to_vec_pretty(&fixture)?;
        digest = Sha256::digest(&bytes).into();
        let path = artifact.join("fixture.json");
        std::fs::write(&path, bytes)?;
        fixture_path = Some(path);
        let solc =
            PathBuf::from(std::env::var_os("EVE_SOLC_BINARY").context("EVE_SOLC_BINARY required")?);
        let mut code = compile_transition_contract(&solc, &artifact.join("solc.json"))?;
        code.extend_from_slice(authority_address.as_slice());
        code.extend_from_slice(b"EVE_B3_ACCEPTANCE_V1");
        code.extend_from_slice(&digest);
        accounts.push(serde_json::json!({
            "address": format!("0x{}", super::TRANSITION_ADDRESS),
            "funded_balance": "0",
            "nonce": 0,
            "code": format!("0x{}", hex::encode(code))
        }));
    }
    let validators: Vec<_> = nodes
        .iter()
        .take(4)
        .map(|node| {
            serde_json::json!({
                "owner": node.owner,
                "classical_public_key": format!("0x{}", hex::encode(node.public_key)),
                "self_bond": "10000000000000000000000",
                "voting_power": 10000
            })
        })
        .collect();
    let document = serde_json::json!({
        "schema_version": 1,
        "protocol_version": 1,
        "network_name": "eve-local-v1",
        "evm_chain_id": 31337,
        "initial_timestamp": 1,
        "profile": "CLASSICAL_DEV",
        "accounts": accounts,
        "validators": validators
    });
    let bytes = serde_json::to_vec_pretty(&document)?;
    let spec = decode_development_spec(&bytes)
        .map_err(|error| anyhow::anyhow!("harness genesis rejected: {error:?}"))?;
    let genesis = initialize_development_state(&spec, &development_state_budget())
        .map_err(|error| anyhow::anyhow!("harness genesis state rejected: {error:?}"))?;
    let path = artifact.join("genesis.json");
    std::fs::write(&path, bytes)?;
    Ok((
        path,
        genesis,
        authority,
        authority_address,
        fixture_path,
        digest,
    ))
}
