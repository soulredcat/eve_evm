// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{summarize_node_failure, write_private_file};
use std::os::unix::fs::{PermissionsExt, symlink};

#[test]
fn cli_failure_metadata_binds_exact_pid_and_redacts_unknown_values() {
    let data = tempfile::tempdir().unwrap();
    let path = data.path().join("node.stderr");
    write_private_file(&path, b"EVE_DEVELOPMENT_VALIDATOR_FAILED 41: ENGINE_NAMESPACE_FAILED\nEVE_DEVELOPMENT_VALIDATOR_FAILED 42: ENGINE_PIDFD_OPEN_FAILED,IO_PERMISSION_DENIED,private-key=never-publish\n").unwrap();
    assert_eq!(
        super::summarize_cli_failure(&path, 42),
        "ENGINE_PIDFD_OPEN_FAILED,IO_PERMISSION_DENIED,REDACTED_CLI_CATEGORY"
    );
    assert_eq!(
        super::summarize_cli_failure(&path, 43),
        "CLI_FAILURE_RECORD_UNAVAILABLE"
    );
}

#[test]
fn cli_failure_metadata_rejects_links_permissions_and_oversized_input() {
    let data = tempfile::tempdir().unwrap();
    let path = data.path().join("node.stderr");
    let other = data.path().join("other.stderr");
    write_private_file(
        &other,
        b"EVE_DEVELOPMENT_VALIDATOR_FAILED 42: ENGINE_PIDFD_OPEN_FAILED\n",
    )
    .unwrap();
    symlink(&other, &path).unwrap();
    assert_eq!(
        super::summarize_cli_failure(&path, 42),
        "CLI_FAILURE_RECORD_UNAVAILABLE"
    );
    std::fs::remove_file(&path).unwrap();
    write_private_file(&path, &vec![b'x'; 16_385]).unwrap();
    assert_eq!(
        super::summarize_cli_failure(&path, 42),
        "CLI_FAILURE_RECORD_UNAVAILABLE"
    );
    std::fs::remove_file(&path).unwrap();
    write_private_file(&path, &std::fs::read(&other).unwrap()).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(
        super::summarize_cli_failure(&path, 42),
        "CLI_FAILURE_RECORD_UNAVAILABLE"
    );
}

#[test]
fn engine_discovery_reports_owned_validator_exit_without_waiting_for_deadline() {
    let data = tempfile::tempdir().unwrap();
    let stderr = data.path().join("node.stderr");
    write_private_file(&stderr, b"").unwrap();
    let mut child = std::process::Command::new("/bin/false").spawn().unwrap();
    child.wait().unwrap();
    let started = std::time::Instant::now();
    let error = super::identify_engine(
        &mut child,
        std::path::Path::new("/bin/false"),
        data.path(),
        data.path(),
        &stderr,
    )
    .unwrap_err();
    assert!(
        error
            .to_string()
            .starts_with("owned validator exited during engine discovery")
    );
    assert!(started.elapsed() < std::time::Duration::from_secs(5));
}

#[test]
fn failure_metadata_releases_only_fixed_categories_and_known_io_kinds() {
    let data = tempfile::tempdir().unwrap();
    let path = data.path().join("runtime-failure-42.log");
    write_private_file(&path, br#"{"version":1,"process_id":42,"categories":["SIGNER_ACTOR","APPROVAL_SIGNING_FAILED","SIGNER_ACTOR","private-key=never-publish"],"io_kind":"WouldBlock","extra_secret":"never-publish"}"#).unwrap();
    assert_eq!(
        summarize_node_failure(data.path(), 42),
        "APPROVAL_SIGNING_FAILED,IO_WOULD_BLOCK,REDACTED_UNCLASSIFIED,SIGNER_ACTOR"
    );
    std::fs::remove_file(&path).unwrap();
    write_private_file(&path, br#"{"version":1,"process_id":42,"categories":["SIGNER_READ"],"io_kind":"secret-never-publish"}"#).unwrap();
    assert_eq!(
        summarize_node_failure(data.path(), 42),
        "REDACTED_IO_KIND,SIGNER_READ"
    );
}

#[test]
fn failure_metadata_rejects_wrong_identity_version_and_shape() {
    let data = tempfile::tempdir().unwrap();
    let path = data.path().join("runtime-failure-42.log");
    for value in [
        serde_json::json!({"version":2,"process_id":42,"categories":["SIGNER_ACTOR"]}),
        serde_json::json!({"version":1,"process_id":43,"categories":["SIGNER_ACTOR"]}),
        serde_json::json!({"version":1,"process_id":42,"categories":"never-publish"}),
        serde_json::json!({"version":1,"process_id":42,"categories":vec!["SIGNER_ACTOR";9]}),
    ] {
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        write_private_file(&path, &serde_json::to_vec(&value).unwrap()).unwrap();
        let summary = summarize_node_failure(data.path(), 42);
        assert!(
            summary == "NODE_FAILURE_RECORD_UNAVAILABLE"
                || summary == "NODE_FAILURE_RECORD_INVALID"
        );
        assert!(!summary.contains("never-publish"));
    }
}

#[test]
fn failure_metadata_never_follows_links_or_reads_oversized_unprivate_files() {
    let data = tempfile::tempdir().unwrap();
    let path = data.path().join("runtime-failure-42.log");
    assert_eq!(
        summarize_node_failure(data.path(), 42),
        "NODE_FAILURE_RECORD_UNAVAILABLE"
    );
    let other = data.path().join("other-private-file");
    write_private_file(
        &other,
        br#"{"version":1,"process_id":42,"categories":["SIGNER_ACTOR"]}"#,
    )
    .unwrap();
    symlink(&other, &path).unwrap();
    assert_eq!(
        summarize_node_failure(data.path(), 42),
        "NODE_FAILURE_RECORD_UNAVAILABLE"
    );
    std::fs::remove_file(&path).unwrap();
    write_private_file(&path, &vec![b'x'; 8194]).unwrap();
    assert_eq!(
        summarize_node_failure(data.path(), 42),
        "NODE_FAILURE_RECORD_UNAVAILABLE"
    );
    std::fs::remove_file(&path).unwrap();
    write_private_file(&path, &std::fs::read(&other).unwrap()).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(
        summarize_node_failure(data.path(), 42),
        "NODE_FAILURE_RECORD_UNAVAILABLE"
    );
}
