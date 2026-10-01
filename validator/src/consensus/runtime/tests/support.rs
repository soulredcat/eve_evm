// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::config::DevelopmentValidatorConfig;
use ed25519_dalek::SigningKey;
use eve_state::development_economics;
use std::{net::TcpListener, os::unix::fs::PermissionsExt, path::Path};

pub(super) fn config(root: &Path) -> DevelopmentValidatorConfig {
    let economics = development_economics();
    let accounts = (1..=4).map(|index| serde_json::json!({"address": format!("0x{}", hex::encode([index;20])), "funded_balance": (economics.validator_self_bond * alloy_primitives::U256::from(10)).to_string(), "nonce": 0, "code":"0x"})).collect::<Vec<_>>();
    let validators = (1..=4).map(|index| serde_json::json!({"owner": format!("0x{}", hex::encode([index;20])), "classical_public_key": format!("0x{}", hex::encode(SigningKey::from_bytes(&[index;32]).verifying_key().to_bytes())), "self_bond": economics.validator_self_bond.to_string(), "voting_power":10_000})).collect::<Vec<_>>();
    let genesis = root.join("public.json");
    std::fs::write(&genesis, serde_json::to_vec(&serde_json::json!({"schema_version":1,"protocol_version":1,"network_name":"eve-local-v1","evm_chain_id":31337,"initial_timestamp":1728000000,"profile":"CLASSICAL_DEV","accounts":accounts,"validators":validators})).unwrap()).unwrap();
    let seed = root.join("seed");
    std::fs::write(&seed, [1; 32]).unwrap();
    std::fs::set_permissions(&seed, std::fs::Permissions::from_mode(0o600)).unwrap();
    let rpc = TcpListener::bind("127.0.0.1:0").unwrap();
    let p2p = TcpListener::bind("127.0.0.1:0").unwrap();
    DevelopmentValidatorConfig {
        genesis,
        data: root.join("node"),
        signing_seed: seed,
        comet_binary: std::env::var_os("EVE_COMET")
            .expect("actual pinned native engine required")
            .into(),
        comet_sha256: std::env::var("EVE_COMET_SHA256")
            .expect("actual native engine digest required"),
        acknowledge_unsafe_development: true,
        rpc_address: rpc.local_addr().unwrap(),
        p2p_address: p2p.local_addr().unwrap(),
        advertised_p2p_address: None,
        acceptance_fixture: None,
        persistent_peers: String::new(),
        zone_id: 1,
    }
}
