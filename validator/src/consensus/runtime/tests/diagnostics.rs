// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::{
    diagnostics::{NodeFailureRecord, SignerPosition, classify_node_failure, write_node_failure},
    supervision::capture_first_channel_failure,
};
use std::{os::unix::fs::PermissionsExt, sync::Mutex};

#[test]
fn bounded_failure_log_is_private_and_never_overwrites_existing_or_foreign_paths() {
    let root = tempfile::tempdir().unwrap();
    std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    let record = NodeFailureRecord {
        version: 1,
        process_id: 42,
        categories: vec!["SIGNER_ACTOR", "HRS_REGRESSION"],
        io_kind: None,
        application_height: Some(8),
        signer_last: Some(SignerPosition {
            height: 9,
            round: 0,
            step: 3,
        }),
        signer_requested: Some(SignerPosition {
            height: 9,
            round: 0,
            step: 2,
        }),
    };
    write_node_failure(root.path(), &record).unwrap();
    let path = root.path().join("runtime-failure-42.log");
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let bytes = std::fs::read(&path).unwrap();
    assert!(bytes.len() <= 8192);
    assert!(write_node_failure(root.path(), &record).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    let foreign = root.path().join("runtime-failure-43.log");
    std::fs::write(&foreign, b"preserve foreign content").unwrap();
    let other = NodeFailureRecord {
        process_id: 43,
        ..record
    };
    assert!(write_node_failure(root.path(), &other).is_err());
    assert_eq!(std::fs::read(foreign).unwrap(), b"preserve foreign content");
}

#[test]
fn diagnostic_categories_redact_dynamic_packet_or_secret_strings() {
    let sensitive = "seed=private-material sign_bytes=raw-packet credential=untrusted";
    let category = classify_node_failure(sensitive);
    assert_eq!(category, "REDACTED_UNCLASSIFIED");
    assert_eq!(
        classify_node_failure("signer height/round/step regression"),
        "HRS_REGRESSION"
    );
    let record = NodeFailureRecord {
        version: 1,
        process_id: 44,
        categories: vec![category],
        io_kind: None,
        application_height: None,
        signer_last: None,
        signer_requested: None,
    };
    let output = serde_json::to_string(&record).unwrap();
    assert!(!output.contains(sensitive));
    assert!(
        !output.contains("private-material")
            && !output.contains("raw-packet")
            && !output.contains("untrusted")
    );
}

#[test]
fn first_actor_failure_survives_secondary_shutdown_errors() {
    let slot = Mutex::new(None);
    capture_first_channel_failure(
        &slot,
        anyhow::anyhow!("signer height/round/step regression"),
    )
    .unwrap();
    capture_first_channel_failure(
        &slot,
        anyhow::anyhow!("secondary closed application stream"),
    )
    .unwrap();
    assert_eq!(
        slot.lock().unwrap().as_ref().unwrap().to_string(),
        "signer height/round/step regression"
    );
}
