use crate::support;

use eve_state::{compute_commit_identity, encode_state_commit};
use eve_storage::state::{
    CommitDisposition, capture_state_snapshot, commit_state, create_state_service,
    open_state_repository, read_snapshot_commit, read_state_service, state_reader,
};
use support::{
    commits::empty_child,
    execution::executed_commit,
    storage::{budget, database_path, local_directory},
};

#[test]
fn ts05_whole_commit_receipts_fees_and_marker_reopen_as_one_identity() {
    let (genesis, next) = executed_commit();
    let directory = local_directory("atomic-state-");
    let path = database_path(&directory);
    let mut repository = open_state_repository(&path, &genesis, budget()).unwrap();
    let expected_bytes = encode_state_commit(&next, &budget().logical).unwrap();
    let identity = compute_commit_identity(&next, &budget().logical).unwrap();
    let ack = commit_state(&mut repository, &next).unwrap();
    assert_eq!(ack.disposition, CommitDisposition::NewlySynced);
    assert_eq!(ack.committed, next.target);
    assert_eq!(ack.store_head, next.target);
    assert_eq!(ack.commit_identity, identity);
    drop(repository);
    let repository = open_state_repository(&path, &genesis, budget()).unwrap();
    let reader = state_reader(&repository);
    let snapshot = capture_state_snapshot(&reader).unwrap();
    let restored = read_snapshot_commit(&snapshot, 1).unwrap().unwrap();
    assert_eq!(restored, next);
    assert_eq!(
        encode_state_commit(&restored, &budget().logical).unwrap(),
        expected_bytes
    );
    assert_eq!(
        compute_commit_identity(&restored, &budget().logical).unwrap(),
        identity
    );
}

#[test]
fn ts05_exact_replay_before_and_after_restart_cannot_duplicate_fee_or_supply_effects() {
    let (genesis, next) = executed_commit();
    let directory = local_directory("replay-state-");
    let path = database_path(&directory);
    let mut repository = open_state_repository(&path, &genesis, budget()).unwrap();
    let first = commit_state(&mut repository, &next).unwrap();
    let replay = commit_state(&mut repository, &next).unwrap();
    assert_eq!(replay.disposition, CommitDisposition::ExactReplay);
    assert_eq!(replay.database_sequence, first.database_sequence);
    assert_eq!(replay.commit_identity, first.commit_identity);
    drop(repository);
    let mut reopened = open_state_repository(&path, &genesis, budget()).unwrap();
    let replay = commit_state(&mut reopened, &next).unwrap();
    assert_eq!(replay.disposition, CommitDisposition::ExactReplay);
    let reader = state_reader(&reopened);
    let snapshot = capture_state_snapshot(&reader).unwrap();
    assert_eq!(read_snapshot_commit(&snapshot, 1).unwrap(), Some(next));
    assert_eq!(read_snapshot_commit(&snapshot, 0).unwrap(), Some(genesis));
}

#[test]
fn ts06_captured_snapshot_and_cached_view_do_not_mix_committed_versions() {
    let (genesis, next) = executed_commit();
    let directory = local_directory("view-state-");
    let mut repository =
        open_state_repository(&database_path(&directory), &genesis, budget()).unwrap();
    let reader = state_reader(&repository);
    let old_snapshot = capture_state_snapshot(&reader).unwrap();
    let service = create_state_service(state_reader(&repository));
    let old_view = read_state_service(&service).unwrap();
    assert_eq!(old_view.commit(), &genesis);
    commit_state(&mut repository, &next).unwrap();
    assert_eq!(
        read_snapshot_commit(&old_snapshot, 0).unwrap(),
        Some(genesis.clone())
    );
    assert_eq!(read_snapshot_commit(&old_snapshot, 1).unwrap(), None);
    assert_eq!(old_view.commit(), &genesis);
    let new_view = read_state_service(&service).unwrap();
    assert_eq!(new_view.commit(), &next);
    assert!(!std::sync::Arc::ptr_eq(&old_view, &new_view));
    assert!(std::sync::Arc::ptr_eq(
        &new_view,
        &read_state_service(&service).unwrap()
    ));
}

#[test]
fn ts05_stale_parent_and_changed_same_height_replay_leave_the_head_unchanged() {
    let (genesis, next) = executed_commit();
    let future = empty_child(&next);
    let directory = local_directory("parent-state-");
    let mut repository =
        open_state_repository(&database_path(&directory), &genesis, budget()).unwrap();
    assert!(commit_state(&mut repository, &future).is_err());
    commit_state(&mut repository, &next).unwrap();
    let mut changed = next.clone();
    changed.block.header.timestamp += 1;
    // This is a new coherent candidate for the same height, not an attempt to
    // rewrite the already-bound current hash inside an existing version.
    changed.state.block_hashes.remove(&changed.target.height);
    let changed = eve_state::build_state_commit(
        changed.parent,
        changed.state,
        changed.block,
        &budget().logical,
    )
    .unwrap();
    assert!(commit_state(&mut repository, &changed).is_err());
    let reader = state_reader(&repository);
    let snapshot = capture_state_snapshot(&reader).unwrap();
    assert_eq!(read_snapshot_commit(&snapshot, 1).unwrap(), Some(next));
    assert_eq!(read_snapshot_commit(&snapshot, 2).unwrap(), None);
}
