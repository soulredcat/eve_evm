mod state_support;
use eve_storage::state::{
    activate_snapshot_namespace, capture_state_snapshot, commit_state,
    development_state_storage_budget, export_state_snapshot, open_state_repository,
    read_snapshot_commit, state_reader,
};

#[test]
fn actual_database_sequence_snapshot_keeps_both_domains_and_recovery_payload_at_one_height() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let first = state_support::next(&genesis, 1);
    let second = state_support::next(&first, 2);
    let mut store = open_state_repository(
        &directory.path().join("db"),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    commit_state(&mut store, &first).unwrap();
    let reader = state_reader(&store);
    let snapshot = capture_state_snapshot(&reader).unwrap();
    let sequence = snapshot.sequence();
    commit_state(&mut store, &second).unwrap();
    assert_eq!(snapshot.version(), &first.target);
    assert_eq!(snapshot.sequence(), sequence);
    assert_eq!(read_snapshot_commit(&snapshot, 1).unwrap(), Some(first));
    assert!(read_snapshot_commit(&snapshot, 2).unwrap().is_none());
}

#[test]
fn complete_snapshot_activates_only_new_namespace_and_preserves_live_source() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let first = state_support::next(&genesis, 1);
    let second = state_support::next(&first, 2);
    let budget = development_state_storage_budget();
    let mut store =
        open_state_repository(&directory.path().join("live"), &genesis, budget).unwrap();
    commit_state(&mut store, &first).unwrap();
    let reader = state_reader(&store);
    let snapshot = capture_state_snapshot(&reader).unwrap();
    let exported = directory.path().join("snapshot");
    let manifest = export_state_snapshot(&snapshot, &exported).unwrap();
    assert_eq!(manifest.height, 1);
    commit_state(&mut store, &second).unwrap();
    let destination = directory.path().join("new-namespace");
    let restored = activate_snapshot_namespace(&exported, &destination, &genesis, budget).unwrap();
    let restored_reader = state_reader(&restored);
    let captured = capture_state_snapshot(&restored_reader).unwrap();
    assert_eq!(read_snapshot_commit(&captured, 1).unwrap(), Some(first));
    assert!(activate_snapshot_namespace(&exported, &destination, &genesis, budget).is_err());
    let live = capture_state_snapshot(&reader).unwrap();
    assert_eq!(read_snapshot_commit(&live, 2).unwrap(), Some(second));
}

#[test]
fn bounded_snapshot_leases_release_after_drop() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let mut budget = development_state_storage_budget();
    budget.maximum_snapshots = 1;
    let store = open_state_repository(directory.path(), &genesis, budget).unwrap();
    let reader = state_reader(&store);
    let snapshot = capture_state_snapshot(&reader).unwrap();
    assert!(capture_state_snapshot(&reader).is_err());
    drop(snapshot);
    assert!(capture_state_snapshot(&reader).is_ok());
}
