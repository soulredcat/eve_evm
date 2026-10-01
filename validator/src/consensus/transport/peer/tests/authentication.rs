// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        EngineChannel, authenticate_engine_peer, engine_peer_pid, ensure_application_engine_peer,
        validate_engine_peer_child,
    },
    spawn_authenticated_test_peer, spawn_test_connection,
};
use std::time::Duration;

#[test]
fn actual_child_socket_credentials_and_pinned_image_yield_live_sealed_peer() {
    let mut fixture = spawn_authenticated_test_peer(EngineChannel::Application);
    assert_eq!(engine_peer_pid(&fixture.peer), fixture.child.id());
    ensure_application_engine_peer(&fixture.peer).unwrap();
    validate_engine_peer_child(&fixture.peer, &mut fixture.child).unwrap();
    assert!(
        fixture.child.try_wait().unwrap().is_none(),
        "pidfd liveness check must not reap"
    );
}

#[test]
fn same_uid_different_actual_process_cannot_claim_expected_child_identity() {
    let mut first = spawn_test_connection("hold");
    let mut second = spawn_test_connection("hold");
    assert_ne!(
        first.child.as_ref().unwrap().id(),
        second.child.as_ref().unwrap().id()
    );
    let result = authenticate_engine_peer(
        first.stream.take().unwrap(),
        second.child.as_mut().unwrap(),
        &first.image,
        EngineChannel::Application,
        Duration::from_secs(5),
        Duration::from_secs(5),
    );
    assert!(result.is_err());
}

#[test]
fn actual_child_with_wrong_verified_executable_digest_cannot_yield_peer() {
    let mut fixture = spawn_test_connection("hold");
    let mut wrong = fixture.expected_sha256;
    wrong[0] ^= 1;
    assert!(
        crate::development::engine::verify_test_engine_image(&fixture.executable, wrong).is_err()
    );
    let other_image = spawn_test_connection("hold");
    assert!(
        authenticate_engine_peer(
            fixture.stream.take().unwrap(),
            fixture.child.as_mut().unwrap(),
            &other_image.image,
            EngineChannel::Application,
            Duration::from_secs(5),
            Duration::from_secs(5)
        )
        .is_err()
    );
}

#[test]
fn exited_child_and_saved_peer_proof_are_rejected_without_pid_reuse_assumptions() {
    let mut raw = spawn_test_connection("hold");
    raw.child.as_mut().unwrap().kill().unwrap();
    raw.child.as_mut().unwrap().wait().unwrap();
    assert!(
        authenticate_engine_peer(
            raw.stream.take().unwrap(),
            raw.child.as_mut().unwrap(),
            &raw.image,
            EngineChannel::Application,
            Duration::from_secs(5),
            Duration::from_secs(5)
        )
        .is_err()
    );
    let mut fixture = spawn_authenticated_test_peer(EngineChannel::Application);
    fixture.child.kill().unwrap();
    fixture.child.wait().unwrap();
    assert!(ensure_application_engine_peer(&fixture.peer).is_err());
    assert!(validate_engine_peer_child(&fixture.peer, &mut fixture.child).is_err());
}

#[test]
fn signer_channel_peer_cannot_authorize_application_callbacks() {
    let fixture = spawn_authenticated_test_peer(EngineChannel::Signer);
    assert_eq!(
        ensure_application_engine_peer(&fixture.peer)
            .unwrap_err()
            .kind(),
        std::io::ErrorKind::PermissionDenied
    );
}

#[test]
fn engine_peer_rejects_zero_unbounded_deadlines_and_zero_launch_digest() {
    for invalid in 0..2 {
        let mut fixture = spawn_test_connection("hold");
        let read = if invalid == 0 {
            Duration::ZERO
        } else if invalid == 1 {
            Duration::from_secs(61)
        } else {
            Duration::from_secs(5)
        };
        assert!(
            authenticate_engine_peer(
                fixture.stream.take().unwrap(),
                fixture.child.as_mut().unwrap(),
                &fixture.image,
                EngineChannel::Application,
                read,
                Duration::from_secs(5)
            )
            .is_err()
        );
    }
    let fixture = spawn_test_connection("hold");
    assert!(
        crate::development::engine::verify_test_engine_image(&fixture.executable, [0; 32]).is_err()
    );
}

#[test]
fn changed_verified_image_metadata_refuses_saved_peer_even_with_unchanged_bytes() {
    use std::os::unix::fs::PermissionsExt;
    let fixture = spawn_authenticated_test_peer(EngineChannel::Application);
    let executable = fixture._directory.path().join("owned-test-worker");
    std::fs::set_permissions(executable, std::fs::Permissions::from_mode(0o500)).unwrap();
    assert!(ensure_application_engine_peer(&fixture.peer).is_err());
}
