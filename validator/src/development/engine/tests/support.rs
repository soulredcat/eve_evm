// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::config::DevelopmentValidatorConfig;
use base64::{Engine, engine::general_purpose::STANDARD};
use eve_consensus_comet::consensus::certificates::validator_address;
use std::{
    net::TcpListener,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

pub(super) fn config(data: &Path) -> (DevelopmentValidatorConfig, [u8; 32]) {
    std::fs::set_permissions(data, std::fs::Permissions::from_mode(0o700)).unwrap();
    let binary = PathBuf::from(
        std::env::var_os("EVE_COMET")
            .expect("pinned EVE_COMET is mandatory; no skipped native tests"),
    );
    let bytes = hex::decode(
        std::env::var("EVE_COMET_SHA256").expect("verified EVE_COMET_SHA256 is mandatory"),
    )
    .unwrap();
    let digest: [u8; 32] = bytes.try_into().unwrap();
    let rpc = TcpListener::bind("127.0.0.1:0").unwrap();
    let p2p = TcpListener::bind("127.0.0.1:0").unwrap();
    (
        DevelopmentValidatorConfig {
            genesis: data.join("public-source.json"),
            data: data.to_owned(),
            signing_seed: data.join("root-signer-secret-is-not-passed"),
            comet_binary: binary,
            comet_sha256: hex::encode(digest),
            acknowledge_unsafe_development: true,
            rpc_address: rpc.local_addr().unwrap(),
            p2p_address: p2p.local_addr().unwrap(),
            advertised_p2p_address: None,
            acceptance_fixture: None,
            persistent_peers: String::new(),
            zone_id: 1,
            recovery_query_working_bytes: 64 * 1_048_576,
        },
        digest,
    )
}

pub(super) fn native_genesis() -> Vec<u8> {
    // Public outputs of deliberately unsafe test-only deterministic keys, never native signing keys.
    let validators = (1..=4).map(|index| {
        let key = ed25519_dalek::SigningKey::from_bytes(&[index; 32]).verifying_key().to_bytes();
        serde_json::json!({"address": hex::encode_upper(validator_address(&key)), "pub_key": {"type": "tendermint/PubKeyEd25519", "value": STANDARD.encode(key)}, "power": "10", "name": format!("test-{index}")})
    }).collect::<Vec<_>>();
    serde_json::to_vec(&serde_json::json!({
        "genesis_time": "1970-01-01T00:00:01Z", "chain_id": "eve-local-v1", "initial_height": "1",
        "consensus_params": {"block": {"max_bytes": "4194304", "max_gas": "30000000"}, "evidence": {"max_age_num_blocks": "1000", "max_age_duration": "86400000000000", "max_bytes": "1048576"}, "validator": {"pub_key_types": ["ed25519"]}, "version": {"app": "1"}, "abci": {"vote_extensions_enable_height": "0"}, "authority": {"authority": ""}},
        "validators": validators, "app_hash": "00".repeat(32), "app_state": {},
    })).unwrap()
}
