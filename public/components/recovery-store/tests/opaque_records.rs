// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod opaque_record_support;
use eve_storage::records::{
    OpaqueRecordDisposition, compare_and_append_opaque_records, opaque_record_cursor,
    open_opaque_record_repository, read_opaque_record,
};
use opaque_record_support::{budget, identity};

#[test]
fn opaque_synced_compare_replay_and_restart_preserve_complete_payload_identity() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("opaque");
    let mut store = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    let initial = opaque_record_cursor(&store).unwrap();
    assert_eq!(initial.sequence, 0);
    assert_ne!(initial.content_hash, [0; 32]);
    let payloads = vec![
        b"complete opaque intent A".to_vec(),
        b"complete opaque intent B".to_vec(),
    ];
    let first = compare_and_append_opaque_records(&mut store, initial, &payloads).unwrap();
    assert_eq!(first.disposition, OpaqueRecordDisposition::NewlySynced);
    assert_eq!(first.appended.sequence, 2);
    let replay = compare_and_append_opaque_records(&mut store, initial, &payloads).unwrap();
    assert_eq!(replay.disposition, OpaqueRecordDisposition::ExactReplay);
    assert_eq!(replay.database_sequence, first.database_sequence);
    let third =
        compare_and_append_opaque_records(&mut store, first.appended, &[b"C".to_vec()]).unwrap();
    let old_replay = compare_and_append_opaque_records(&mut store, initial, &payloads).unwrap();
    assert_eq!(old_replay.appended, first.appended);
    assert_eq!(old_replay.store_head, third.appended);
    assert_eq!(old_replay.database_sequence, third.database_sequence);
    let mut changed = payloads.clone();
    changed[0][0] ^= 1;
    assert!(compare_and_append_opaque_records(&mut store, initial, &changed).is_err());
    let mut mixed = payloads.clone();
    mixed.extend([b"C".to_vec(), b"D".to_vec()]);
    assert!(compare_and_append_opaque_records(&mut store, initial, &mixed).is_err());
    assert_eq!(opaque_record_cursor(&store).unwrap(), third.store_head);
    drop(store);
    let reopened = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), third.store_head);
    for (index, payload) in payloads.iter().enumerate() {
        let record = read_opaque_record(&reopened, (index + 1) as u64)
            .unwrap()
            .unwrap();
        assert_eq!(&record.payload, payload);
    }
    assert!(read_opaque_record(&reopened, 0).unwrap().is_none());
    assert!(read_opaque_record(&reopened, 4).unwrap().is_none());
}

#[test]
fn opaque_network_owner_and_domain_cannot_rebind_existing_namespace() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("opaque");
    let mut store = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    let initial = opaque_record_cursor(&store).unwrap();
    let ack = compare_and_append_opaque_records(&mut store, initial, &[vec![7]]).unwrap();
    drop(store);
    for field in 0..3 {
        let mut changed = identity();
        match field {
            0 => changed.genesis_hash[0] ^= 1,
            1 => changed.owner[0] ^= 1,
            _ => changed.domain[0] ^= 1,
        }
        assert!(open_opaque_record_repository(&path, changed, budget()).is_err());
    }
    let reopened = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), ack.store_head);
}

#[test]
fn opaque_stale_or_future_compare_rejects_without_mutating_valid_owner() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("opaque");
    let mut store = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    let initial = opaque_record_cursor(&store).unwrap();
    let mut changed = initial;
    changed.content_hash[0] ^= 1;
    assert!(compare_and_append_opaque_records(&mut store, changed, &[vec![1]]).is_err());
    changed = initial;
    changed.sequence = 1;
    assert!(compare_and_append_opaque_records(&mut store, changed, &[vec![1]]).is_err());
    assert!(compare_and_append_opaque_records(&mut store, initial, &[]).is_err());
    assert_eq!(opaque_record_cursor(&store).unwrap(), initial);
    assert!(compare_and_append_opaque_records(&mut store, initial, &[vec![1]]).is_ok());
}

#[test]
fn opaque_initialization_rejects_existing_empty_foreign_and_missing_parent_directories() {
    let temporary = tempfile::tempdir().unwrap();
    let empty = temporary.path().join("empty");
    std::fs::create_dir(&empty).unwrap();
    assert!(open_opaque_record_repository(&empty, identity(), budget()).is_err());
    let absent_parent = temporary.path().join("absent-parent/opaque");
    assert!(open_opaque_record_repository(&absent_parent, identity(), budget()).is_err());
    assert!(!absent_parent.parent().unwrap().exists());
    let foreign = temporary.path().join("foreign");
    let raw = rocksdb::DB::open_default(&foreign).unwrap();
    raw.put(b"other/domain", b"preserve me").unwrap();
    drop(raw);
    assert!(open_opaque_record_repository(&foreign, identity(), budget()).is_err());
    let raw = rocksdb::DB::open_default(&foreign).unwrap();
    assert_eq!(raw.get(b"other/domain").unwrap().unwrap(), b"preserve me");
    assert!(raw.get(b"eve/opaque/v1/schema").unwrap().is_none());
}

#[test]
fn opaque_complete_older_checkpoint_is_not_a_general_backup_rollback_detector() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("current");
    let older = temporary.path().join("coherent-older-copy");
    let mut store = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    let initial = opaque_record_cursor(&store).unwrap();
    let first = compare_and_append_opaque_records(&mut store, initial, &[vec![1]]).unwrap();
    drop(store);
    let raw = rocksdb::DB::open_default(&path).unwrap();
    rocksdb::checkpoint::Checkpoint::new(&raw)
        .unwrap()
        .create_checkpoint(&older)
        .unwrap();
    drop(raw);
    let mut store = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    let second =
        compare_and_append_opaque_records(&mut store, first.store_head, &[vec![2]]).unwrap();
    let old_copy = open_opaque_record_repository(&older, identity(), budget()).unwrap();
    assert_eq!(opaque_record_cursor(&old_copy).unwrap(), first.store_head);
    assert_ne!(opaque_record_cursor(&old_copy).unwrap(), second.store_head);
    // A coherent copied namespace is a different filesystem lock: caller/key continuity must reject it.
}
