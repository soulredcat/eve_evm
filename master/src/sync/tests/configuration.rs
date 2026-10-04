// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{fixture, proof_directory};
use crate::sync::{current_master_commit, import_master_wire, open_master_follower};
use eve_state::SecurityProfile;

#[test]
fn role_profile_acknowledgement_and_actual_working_envelope_reject_before_namespace_creation() {
    let fixture = fixture();
    for (mode, acknowledged) in [
        ("MASTER_SYNC_ONLY", false),
        ("DEV_ALL_IN_ONE", true),
        ("PRODUCTION", true),
        ("", true),
    ] {
        let mut config = fixture.config.clone();
        config.mode = mode.into();
        config.acknowledge_unsafe_development = acknowledged;
        assert!(open_master_follower(config, &fixture.chain.genesis).is_err());
        assert!(!proof_directory(&fixture).exists());
    }
    let mut genesis = fixture.chain.genesis.clone();
    genesis.profile = SecurityProfile::HybridExperimental;
    assert!(open_master_follower(fixture.config.clone(), &genesis).is_err());
    assert!(!proof_directory(&fixture).exists());
    let mut insufficient = fixture.config.clone();
    insufficient.working_bytes = 1_024;
    assert!(open_master_follower(insufficient, &fixture.chain.genesis).is_err());
    assert!(!proof_directory(&fixture).exists());
    let mut overflow = fixture.config.clone();
    overflow.storage.logical.maximum_accounts = usize::MAX;
    assert!(open_master_follower(overflow, &fixture.chain.genesis).is_err());
    assert!(!proof_directory(&fixture).exists());
}

#[test]
fn valid_one_proof_capacity_preserves_the_confirmed_prefix_and_never_prunes_for_next_admission() {
    let fixture = fixture();
    let mut config = fixture.config.clone();
    config.maximum_proof_files = 1;
    let mut follower = open_master_follower(config.clone(), &fixture.chain.genesis).unwrap();
    import_master_wire(
        &mut follower,
        &fixture.wires[0],
        &fixture.chain.commits[1].target,
    )
    .unwrap();
    let original =
        std::fs::read(proof_directory(&fixture).join("00000000000000000001.proof")).unwrap();
    assert!(
        import_master_wire(
            &mut follower,
            &fixture.wires[1],
            &fixture.chain.commits[2].target
        )
        .is_err()
    );
    assert_eq!(current_master_commit(&follower), &fixture.chain.commits[1]);
    assert!(!follower.fenced);
    assert_eq!(
        std::fs::read(proof_directory(&fixture).join("00000000000000000001.proof")).unwrap(),
        original
    );
    assert!(!proof_directory(&fixture).join("pending.proof").exists());
    drop(follower);
    assert_eq!(
        current_master_commit(&open_master_follower(config, &fixture.chain.genesis).unwrap()),
        &fixture.chain.commits[1]
    );
}

#[test]
fn real_pool_pressure_and_target_mismatch_release_transient_capacity_before_any_disk_write() {
    let fixture = fixture();
    let mut follower =
        open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    let available = follower.pool.available_permits();
    let pressure = follower
        .pool
        .clone()
        .try_acquire_many_owned(available as u32)
        .unwrap();
    assert!(
        import_master_wire(
            &mut follower,
            &fixture.wires[0],
            &fixture.chain.commits[1].target
        )
        .is_err()
    );
    assert!(!proof_directory(&fixture).join("pending.proof").exists());
    drop(pressure);
    assert_eq!(follower.pool.available_permits(), available);
    let mut wrong_target = fixture.chain.commits[1].target.clone();
    wrong_target.content_digest.0[0] ^= 1;
    assert!(import_master_wire(&mut follower, &fixture.wires[0], &wrong_target).is_err());
    assert_eq!(follower.pool.available_permits(), available);
    assert!(!proof_directory(&fixture).join("pending.proof").exists());
    assert!(!follower.fenced);
    import_master_wire(
        &mut follower,
        &fixture.wires[0],
        &fixture.chain.commits[1].target,
    )
    .unwrap();
}

#[test]
fn zero_height_never_contacts_a_peer_or_creates_authentication_and_invalid_source_still_rejects() {
    use crate::sync::{follow_master_to_height, master_sync_status};
    use eve_sync_client::NativeRpcConfig;
    use std::time::Duration;
    let fixture = fixture();
    let mut follower =
        open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    let rpc = NativeRpcConfig {
        address: "127.0.0.1:1".parse().unwrap(),
        timeout: Duration::from_millis(10),
        maximum_request_bytes: 131_072,
        maximum_response_bytes: 65_536,
    };
    let status = follow_master_to_height(&mut follower, rpc, 65_536, 0).unwrap();
    assert_eq!(status.applied_height, 0);
    assert!(!status.authenticated_validator_finality && !status.ready);
    assert!(follow_master_to_height(&mut follower, rpc, 0, 0).is_err());
    assert!(
        follow_master_to_height(
            &mut follower,
            rpc,
            65_536,
            fixture.config.maximum_proof_files + 1
        )
        .is_err()
    );
    let mut external = rpc;
    external.address = "192.0.2.1:26657".parse().unwrap();
    assert!(follow_master_to_height(&mut follower, external, 65_536, 0).is_err());
    assert_eq!(master_sync_status(&follower).retained_proof_files, 0);
    assert!(!follower.fenced);
}
