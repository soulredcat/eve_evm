// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::init_development_validator;

#[test]
fn actual_native_initialization_reopens_same_public_identity_without_voting() {
    let root = tempfile::tempdir().unwrap();
    let config = super::support::config(root.path());
    let first = init_development_validator(&config).unwrap();
    let second = init_development_validator(&config).unwrap();
    assert_eq!(first.node_id, second.node_id);
    assert_eq!(first.genesis_hash, second.genesis_hash);
    assert_eq!(first.security_profile, "CLASSICAL_DEV");
    assert_eq!(first.node_id.len(), 40);
    assert!(std::net::TcpStream::connect(config.rpc_address).is_err());
    assert!(std::net::TcpStream::connect(config.p2p_address).is_err());
    let json = serde_json::to_string(&first).unwrap();
    assert!(!json.contains("signing_seed") && !json.contains("private_key"));
}

#[test]
fn explicit_development_acknowledgment_and_enrollment_are_required() {
    let root = tempfile::tempdir().unwrap();
    let mut config = super::support::config(root.path());
    config.acknowledge_unsafe_development = false;
    assert!(init_development_validator(&config).is_err());
    assert!(!config.data.exists());
    config.acknowledge_unsafe_development = true;
    std::fs::write(&config.signing_seed, [9; 32]).unwrap();
    assert!(init_development_validator(&config).is_err());
    assert!(!config.data.exists());
}
