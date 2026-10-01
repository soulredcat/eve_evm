// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support;

use eve_storage::state::{
    activate_snapshot_namespace, capture_state_snapshot, commit_state, export_state_snapshot,
    open_state_repository, read_snapshot_commit, state_reader,
};
use support::{
    execution::executed_commit,
    storage::{budget, database_path, local_directory},
};

#[test]
fn ts06_snapshot_exports_one_captured_version_and_activates_only_a_new_namespace() {
    let (genesis, next) = executed_commit();
    let directory = local_directory("snapshot-transfer-");
    let mut repository =
        open_state_repository(&database_path(&directory), &genesis, budget()).unwrap();
    let reader = state_reader(&repository);
    let old = capture_state_snapshot(&reader).unwrap();
    commit_state(&mut repository, &next).unwrap();
    let old_path = directory.path().join("old-export");
    let manifest = export_state_snapshot(&old, &old_path).unwrap();
    assert!(manifest.complete);
    assert_eq!(manifest.height, 0);
    let restored = activate_snapshot_namespace(
        &old_path,
        &directory.path().join("old-restored"),
        &genesis,
        budget(),
    )
    .unwrap();
    let restored_reader = state_reader(&restored);
    let restored_snapshot = capture_state_snapshot(&restored_reader).unwrap();
    assert_eq!(
        read_snapshot_commit(&restored_snapshot, 0).unwrap(),
        Some(genesis.clone())
    );
    assert_eq!(read_snapshot_commit(&restored_snapshot, 1).unwrap(), None);
    let current = capture_state_snapshot(&reader).unwrap();
    let new_path = directory.path().join("new-export");
    assert_eq!(
        export_state_snapshot(&current, &new_path).unwrap().height,
        1
    );
    let new = activate_snapshot_namespace(
        &new_path,
        &directory.path().join("new-restored"),
        &genesis,
        budget(),
    )
    .unwrap();
    let new_reader = state_reader(&new);
    let new_snapshot = capture_state_snapshot(&new_reader).unwrap();
    assert_eq!(read_snapshot_commit(&new_snapshot, 1).unwrap(), Some(next));
    let existing = directory.path().join("existing");
    std::fs::create_dir(&existing).unwrap();
    std::fs::write(existing.join("preserve.txt"), "owner data").unwrap();
    assert!(activate_snapshot_namespace(&new_path, &existing, &genesis, budget()).is_err());
    assert_eq!(
        std::fs::read_to_string(existing.join("preserve.txt")).unwrap(),
        "owner data"
    );
}

#[test]
fn ts05_incomplete_or_corrupt_staged_snapshot_never_replaces_live_state() {
    let (genesis, next) = executed_commit();
    for fault in [
        "missing-manifest",
        "missing-frame",
        "corrupt-frame",
        "unsafe-path",
    ] {
        let directory = local_directory("snapshot-corruption-");
        let mut repository =
            open_state_repository(&database_path(&directory), &genesis, budget()).unwrap();
        commit_state(&mut repository, &next).unwrap();
        let reader = state_reader(&repository);
        let current = capture_state_snapshot(&reader).unwrap();
        let exported = directory.path().join("exported");
        let manifest = export_state_snapshot(&current, &exported).unwrap();
        let frame = exported.join(&manifest.references[1].file_name);
        match fault {
            "missing-manifest" => std::fs::remove_file(exported.join("manifest.json")).unwrap(),
            "missing-frame" => std::fs::remove_file(&frame).unwrap(),
            "corrupt-frame" => {
                let mut bytes = std::fs::read(&frame).unwrap();
                bytes[0] ^= 1;
                std::fs::write(&frame, bytes).unwrap();
            }
            "unsafe-path" => {
                let mut altered = manifest;
                altered.references[0].file_name = "../foreign.rlp".into();
                std::fs::write(
                    exported.join("manifest.json"),
                    serde_json::to_vec(&altered).unwrap(),
                )
                .unwrap();
            }
            _ => unreachable!(),
        }
        let destination = directory.path().join("rejected");
        assert!(
            activate_snapshot_namespace(&exported, &destination, &genesis, budget()).is_err(),
            "{fault}"
        );
        assert!(!destination.exists());
        assert_eq!(
            read_snapshot_commit(&current, 1).unwrap(),
            Some(next.clone())
        );
    }
}
