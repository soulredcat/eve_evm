// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{config, native_genesis};
use crate::development::engine::{engine_node_id, initialize_engine_home};

#[test]
fn actual_native_init_version_digest_node_id_and_recognized_reopen_preserve_genesis() {
    let directory = tempfile::tempdir().unwrap();
    let (config, digest) = config(directory.path());
    let home = directory.path().join("engine");
    let genesis = native_genesis();
    initialize_engine_home(&config, &home, &genesis, digest).unwrap();
    let node = engine_node_id(&config, &home, digest).unwrap();
    assert_eq!(node.len(), 40);
    let key = std::fs::read(home.join("config/node_key.json")).unwrap();
    initialize_engine_home(&config, &home, &genesis, digest).unwrap();
    assert_eq!(engine_node_id(&config, &home, digest).unwrap(), node);
    assert_eq!(
        std::fs::read(home.join("config/node_key.json")).unwrap(),
        key
    );
    assert_eq!(
        std::fs::read(home.join("config/genesis.json")).unwrap(),
        genesis
    );
    assert!(
        !config.signing_seed.exists(),
        "engine never reads or passes the root signer key"
    );
}

#[test]
fn native_digest_and_frozen_genesis_mismatches_reject_before_initialization() {
    let directory = tempfile::tempdir().unwrap();
    let (config, mut digest) = config(directory.path());
    let home = directory.path().join("engine");
    digest[0] ^= 1;
    assert!(initialize_engine_home(&config, &home, &native_genesis(), digest).is_err());
    assert!(!home.exists());
    let mut genesis: serde_json::Value = serde_json::from_slice(&native_genesis()).unwrap();
    genesis["consensus_params"]["block"]["max_gas"] = "30000001".into();
    assert!(
        initialize_engine_home(
            &config,
            &home,
            &serde_json::to_vec(&genesis).unwrap(),
            digest
        )
        .is_err()
    );
    assert!(!home.exists());
}

#[test]
fn version_text_spoof_cannot_execute_without_the_verified_binary_digest() {
    use std::os::unix::fs::PermissionsExt;
    let directory = tempfile::tempdir().unwrap();
    let (mut config, digest) = config(directory.path());
    let fake = directory.path().join("unverified-version-spoof");
    let marker = directory.path().join("must-not-execute");
    std::fs::write(&fake, format!("#!/bin/sh\nprintf '%s\\n' '0.39.0+0880b4d378f347ab16e54ec677ff50d803f37d62'\ntouch '{}'\n", marker.display())).unwrap();
    std::fs::set_permissions(&fake, std::fs::Permissions::from_mode(0o700)).unwrap();
    config.comet_binary = fake;
    assert!(
        initialize_engine_home(
            &config,
            &directory.path().join("engine"),
            &native_genesis(),
            digest
        )
        .is_err()
    );
    assert!(
        !marker.exists(),
        "a version-only spoof must fail before execution"
    );
}

#[test]
fn existing_empty_incomplete_or_changed_engine_homes_never_reset_user_data() {
    let directory = tempfile::tempdir().unwrap();
    let (config, digest) = config(directory.path());
    let empty = directory.path().join("empty");
    std::fs::create_dir(&empty).unwrap();
    std::fs::write(empty.join("preserve-user-data"), b"untouched").unwrap();
    assert!(initialize_engine_home(&config, &empty, &native_genesis(), digest).is_err());
    assert_eq!(
        std::fs::read(empty.join("preserve-user-data")).unwrap(),
        b"untouched"
    );
    let home = directory.path().join("engine");
    initialize_engine_home(&config, &home, &native_genesis(), digest).unwrap();
    let source = home.join("config/genesis.json");
    std::fs::write(&source, b"changed genesis").unwrap();
    assert!(initialize_engine_home(&config, &home, &native_genesis(), digest).is_err());
    assert_eq!(std::fs::read(source).unwrap(), b"changed genesis");
}

#[test]
fn recognized_home_rejects_symlinked_subdirectories_and_provenance_files_without_cleanup() {
    use std::os::unix::fs::symlink;
    let directory = tempfile::tempdir().unwrap();
    let (config, digest) = config(directory.path());
    let home = directory.path().join("engine");
    initialize_engine_home(&config, &home, &native_genesis(), digest).unwrap();
    for relative in [
        "config",
        "data",
        ".eve-engine.json",
        "config/genesis.json",
        "config/config.toml",
    ] {
        let original = home.join(relative);
        let saved = directory.path().join("preserved-path");
        std::fs::rename(&original, &saved).unwrap();
        symlink(&saved, &original).unwrap();
        assert!(initialize_engine_home(&config, &home, &native_genesis(), digest).is_err());
        assert!(
            std::fs::symlink_metadata(&original)
                .unwrap()
                .file_type()
                .is_symlink()
        );
        assert!(saved.exists());
        std::fs::remove_file(&original).unwrap();
        std::fs::rename(&saved, &original).unwrap();
    }
    initialize_engine_home(&config, &home, &native_genesis(), digest).unwrap();
}
