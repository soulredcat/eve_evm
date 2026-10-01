// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{config, native_genesis};
use crate::development::engine::{
    configuration::{configure_engine_home, validate_persistent_engine_peers},
    home::acquire_engine_lease,
    initialize_engine_home,
    verification::verify_engine_binary,
};
use std::os::unix::net::UnixListener;

#[test]
fn native_operational_configuration_enforces_unix_external_signer_and_bounded_local_peers() {
    let directory = tempfile::tempdir().unwrap();
    let (mut config, digest) = config(directory.path());
    config.persistent_peers = format!("{}@127.0.0.1:36656", "ab".repeat(20));
    let home = directory.path().join("engine");
    initialize_engine_home(&config, &home, &native_genesis(), digest).unwrap();
    let app = directory.path().join("application.sock");
    let _listener = UnixListener::bind(&app).unwrap();
    let signer = directory.path().join("native-signer.sock");
    let _lease = acquire_engine_lease(directory.path()).unwrap();
    let binary = verify_engine_binary(&config, digest).unwrap();
    configure_engine_home(&config, &home, &app, &signer, &binary).unwrap();
    let native: toml::Value =
        toml::from_str(&std::fs::read_to_string(home.join("config/config.toml")).unwrap()).unwrap();
    assert_eq!(
        native["priv_validator_laddr"].as_str().unwrap(),
        format!("unix://{}", signer.display())
    );
    assert_eq!(
        native["proxy_app"].as_str().unwrap(),
        format!("unix://{}", app.display())
    );
    assert_eq!(native["p2p"]["pex"].as_bool(), Some(false));
    assert_eq!(native["p2p"]["allow_duplicate_ip"].as_bool(), Some(true));
    assert_eq!(native["rpc"]["unsafe"].as_bool(), Some(false));
    assert_eq!(
        native["mempool"]["max_tx_bytes"].as_integer(),
        Some(131_072)
    );
    assert_eq!(
        native["mempool"]["max_txs_bytes"].as_integer(),
        Some(16 * 1_048_576)
    );
    assert_eq!(
        native["priv_validator_key_file"].as_str(),
        Some("config/priv_validator_key.json")
    );
}

#[test]
fn engine_peer_syntax_count_duplicate_or_remote_targets_fail_closed() {
    for peers in [
        "bad",
        "abcd@127.0.0.1:1",
        &format!("{}@192.0.2.1:26656", "ab".repeat(20)),
        &format!("{}@127.0.0.1:0", "ab".repeat(20)),
        &format!(
            "{}@127.0.0.1:1,{}@127.0.0.1:2",
            "ab".repeat(20),
            "ab".repeat(20)
        ),
    ] {
        assert!(validate_persistent_engine_peers(peers).is_err());
    }
    let peers = (0..17)
        .map(|index| format!("{index:040x}@127.0.0.1:26656"))
        .collect::<Vec<_>>()
        .join(",");
    assert!(validate_persistent_engine_peers(&peers).is_err());
}

#[test]
fn actual_home_os_lease_refuses_a_second_owner_and_releases_without_marker_deletion() {
    let directory = tempfile::tempdir().unwrap();
    let _ = config(directory.path());
    let lease = acquire_engine_lease(directory.path()).unwrap();
    assert!(acquire_engine_lease(directory.path()).is_err());
    drop(lease);
    assert!(acquire_engine_lease(directory.path()).is_ok());
    assert!(directory.path().join(".eve-engine-owner.lock").exists());
}
