// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{config, native_genesis};
use crate::development::engine::{
    engine_child, engine_pid, initialize_engine_home, start_engine, stop_engine,
};
use std::{
    os::unix::net::UnixListener,
    time::{Duration, Instant},
};

#[test]
fn actual_owned_engine_abrupt_stop_reaps_child_and_only_cleans_designated_socket() {
    let directory = tempfile::tempdir().unwrap();
    let (config, digest) = config(directory.path());
    let home = directory.path().join("engine");
    initialize_engine_home(&config, &home, &native_genesis(), digest).unwrap();
    let application = directory.path().join("application.sock");
    let _listener = UnixListener::bind(&application).unwrap();
    let signer = directory.path().join("native-signer.sock");
    let mut engine = start_engine(&config, &home, &application, &signer, digest).unwrap();
    let pid = engine_pid(&engine).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !signer.exists() && Instant::now() < deadline {
        assert!(
            engine_child(&mut engine)
                .unwrap()
                .try_wait()
                .unwrap()
                .is_none()
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(
        signer.exists(),
        "actual native external-signer listener must be created"
    );
    engine_child(&mut engine).unwrap().kill().unwrap();
    stop_engine(&mut engine).unwrap();
    assert_eq!(engine_pid(&engine), None);
    assert!(!std::path::Path::new(&format!("/proc/{pid}")).exists());
    assert!(!signer.exists());
    assert!(
        application.exists(),
        "application listener is root-owned, not engine cleanup"
    );
    assert!(home.join("config/priv_validator_key.json").exists());
    stop_engine(&mut engine).unwrap();
}

#[test]
fn foreign_regular_signer_path_is_never_removed_or_overwritten() {
    let directory = tempfile::tempdir().unwrap();
    let (config, digest) = config(directory.path());
    let home = directory.path().join("engine");
    initialize_engine_home(&config, &home, &native_genesis(), digest).unwrap();
    let application = directory.path().join("application.sock");
    let _listener = UnixListener::bind(&application).unwrap();
    let signer = directory.path().join("native-signer.sock");
    std::fs::write(&signer, b"preserve unrelated file").unwrap();
    assert!(start_engine(&config, &home, &application, &signer, digest).is_err());
    assert_eq!(std::fs::read(&signer).unwrap(), b"preserve unrelated file");
}

#[test]
fn owned_child_cleanup_preserves_a_foreign_replacement_after_abrupt_exit() {
    let directory = tempfile::tempdir().unwrap();
    let (config, digest) = config(directory.path());
    let home = directory.path().join("engine");
    initialize_engine_home(&config, &home, &native_genesis(), digest).unwrap();
    let application = directory.path().join("application.sock");
    let _listener = UnixListener::bind(&application).unwrap();
    let signer = directory.path().join("native-signer.sock");
    let mut engine = start_engine(&config, &home, &application, &signer, digest).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !signer.exists() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(signer.exists());
    engine_child(&mut engine).unwrap().kill().unwrap();
    // Replace only this fixture's ephemeral endpoint; no engine state or key files are removed.
    std::fs::remove_file(&signer).unwrap();
    std::fs::write(&signer, b"preserve foreign replacement").unwrap();
    assert!(stop_engine(&mut engine).is_err());
    assert_eq!(engine_pid(&engine), None);
    assert_eq!(
        std::fs::read(&signer).unwrap(),
        b"preserve foreign replacement"
    );
}
