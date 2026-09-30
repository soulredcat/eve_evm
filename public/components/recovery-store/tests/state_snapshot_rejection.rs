mod state_support;
use eve_storage::state::{
    activate_snapshot_namespace, capture_state_snapshot, commit_state,
    development_state_storage_budget, export_state_snapshot, open_state_repository, state_reader,
};

#[test]
fn interrupted_capacity_limited_import_remains_inactive_and_does_not_touch_source() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let mut first = state_support::next(&genesis, 1);
    for slot in 1u64..=1000 {
        first
            .state
            .accounts
            .get_mut(&eve_state::Address::repeat_byte(5))
            .unwrap()
            .storage
            .insert(eve_state::U256::from(slot), eve_state::U256::from(slot));
    }
    first.block.header.state_root = eve_state::compute_evm_root(&first.state.accounts).0;
    first = eve_state::build_state_commit(
        first.parent,
        first.state,
        first.block,
        &eve_state::development_state_budget(),
    )
    .unwrap();
    let budget = development_state_storage_budget();
    let mut live = open_state_repository(&directory.path().join("live"), &genesis, budget).unwrap();
    commit_state(&mut live, &first).unwrap();
    let reader = state_reader(&live);
    let snapshot = capture_state_snapshot(&reader).unwrap();
    let source = directory.path().join("export");
    export_state_snapshot(&snapshot, &source).unwrap();
    let mut limited = budget;
    limited.maximum_commit_bytes = 16_384;
    let staging = directory.path().join("incomplete");
    assert!(activate_snapshot_namespace(&source, &staging, &genesis, limited).is_err());
    assert!(staging.exists());
    assert!(open_state_repository(&staging, &genesis, budget).is_err());
    assert_eq!(read_state(&reader), first.target);
}

fn read_state(reader: &eve_storage::state::StateReader) -> eve_state::StateVersion {
    capture_state_snapshot(reader).unwrap().version().clone()
}

#[test]
fn checksum_missing_payload_and_partial_export_never_replace_source_or_existing_data() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let budget = development_state_storage_budget();
    let store = open_state_repository(&directory.path().join("live"), &genesis, budget).unwrap();
    let reader = state_reader(&store);
    let captured = capture_state_snapshot(&reader).unwrap();
    let source = directory.path().join("export");
    export_state_snapshot(&captured, &source).unwrap();
    let payload = source.join("commit-00000000000000000000.rlp");
    let original = std::fs::read(&payload).unwrap();
    let mut corrupted = original.clone();
    corrupted[0] ^= 1;
    std::fs::write(&payload, corrupted).unwrap();
    let target = directory.path().join("new");
    assert!(activate_snapshot_namespace(&source, &target, &genesis, budget).is_err());
    assert!(!target.exists());
    std::fs::write(&payload, original).unwrap();
    std::fs::remove_file(&payload).unwrap();
    assert!(activate_snapshot_namespace(&source, &target, &genesis, budget).is_err());
    let partial = directory.path().join("partial");
    std::fs::create_dir(&partial).unwrap();
    std::fs::write(partial.join("partial.rlp"), b"unfinished local export").unwrap();
    assert!(activate_snapshot_namespace(&partial, &target, &genesis, budget).is_err());
    assert!(export_state_snapshot(&captured, &source).is_err());
    assert_eq!(captured.version(), &genesis.target);
}
