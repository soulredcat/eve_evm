// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support;

use alloy_primitives::{Bytes, keccak256};
use eve_state::build_state_commit;
use eve_storage::state::{
    activate_snapshot_namespace, capture_state_snapshot, commit_state, export_state_snapshot,
    open_state_repository, read_snapshot_commit, state_reader,
};
use support::{
    commits::{empty_child, structural_genesis},
    storage::{budget, database_path, local_directory},
};

#[test]
fn ts05_real_export_budget_interruption_retains_partial_frames_without_ready_manifest() {
    let mut settings = budget();
    settings.logical.maximum_state_bytes = 4 * 1024 * 1024;
    settings.logical.maximum_commit_bytes = 4 * 1024 * 1024;
    settings.logical.maximum_total_code_bytes = 4 * 1024 * 1024;
    settings.logical.maximum_system_bytes = 1024 * 1024;
    settings.database.max_record_bytes = 4 * 1024 * 1024;
    settings.database.max_batch_bytes = 8 * 1024 * 1024;
    settings.maximum_commit_bytes = 8 * 1024 * 1024;
    settings.maximum_snapshot_bytes = 8 * 1024 * 1024;
    let mut genesis = structural_genesis("execution_parent");
    for value in 0..100_u8 {
        let code = Bytes::from(vec![value; 24_576]);
        genesis.state.codes.insert(keccak256(&code), code);
    }
    genesis = build_state_commit(None, genesis.state, genesis.block, &settings.logical).unwrap();
    let directory = local_directory("snapshot-interruption-");
    let mut repository =
        open_state_repository(&database_path(&directory), &genesis, settings).unwrap();
    let mut head = genesis.clone();
    for _ in 0..3 {
        head = empty_child(&head);
        commit_state(&mut repository, &head).unwrap();
    }
    let reader = state_reader(&repository);
    let snapshot = capture_state_snapshot(&reader).unwrap();
    let target = directory.path().join("partial");
    // This is a real bounded-export failure, not a mocked fsync or hardware loss.
    assert!(export_state_snapshot(&snapshot, &target).is_err());
    assert!(target.exists());
    assert!(!target.join("manifest.json").exists());
    assert!(std::fs::read_dir(&target).unwrap().next().is_some());
    let destination = directory.path().join("must-not-activate");
    assert!(activate_snapshot_namespace(&target, &destination, &genesis, settings).is_err());
    assert!(!destination.exists());
    assert_eq!(
        read_snapshot_commit(&snapshot, head.target.height).unwrap(),
        Some(head)
    );
    assert_eq!(read_snapshot_commit(&snapshot, 0).unwrap(), Some(genesis));
}
