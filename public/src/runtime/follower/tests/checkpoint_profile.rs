// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::checkpoints::{
    build_checkpoint_recovery_configuration, check_checkpoint_bootstrap_deadline,
    checkpoint_rpc_before,
};
use std::{
    path::Path,
    time::{Duration, Instant},
};

#[test]
fn checkpoint_profile_remains_bounded_and_within_actual_wire_and_state_caps() {
    let config = build_checkpoint_recovery_configuration(Path::new("unused-data"));
    assert_eq!(
        config.content_root,
        Path::new("unused-data/.checkpoints/content")
    );
    assert_eq!(
        config.proof_root,
        Path::new("unused-data/.checkpoints/proofs")
    );
    assert_eq!(config.maximum_scan_records, 100_000);
    assert_eq!(config.limits.content.maximum_body_bytes, 32 * 1_048_576);
    assert_eq!(config.limits.content.maximum_manifest_bytes, 262_144);
    assert_eq!(config.limits.content.maximum_chunk_bytes, 32_768);
    assert_eq!(config.limits.proofs.maximum_files, 10_001);
    assert_eq!(config.limits.proofs.maximum_disk_bytes, 132 * 1_048_576);
    assert_eq!(
        config.limits.content.logical,
        eve_state::development_state_budget()
    );
}

#[test]
fn an_expired_whole_bootstrap_deadline_cannot_create_a_fresh_network_timeout() {
    let elapsed = Instant::now() - Duration::from_millis(1);
    assert!(check_checkpoint_bootstrap_deadline(elapsed).is_err());
    assert!(checkpoint_rpc_before("127.0.0.1:26657".parse().unwrap(), elapsed).is_err());
    let rpc = checkpoint_rpc_before(
        "127.0.0.1:26657".parse().unwrap(),
        Instant::now() + Duration::from_millis(500),
    )
    .unwrap();
    assert!(rpc.timeout <= Duration::from_millis(500));
    assert_eq!(rpc.maximum_response_bytes, 262_144);
}
