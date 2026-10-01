// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod opaque_record_support;
use eve_storage::records::{
    compare_and_append_opaque_records, opaque_record_cursor, open_opaque_record_repository,
};
use opaque_record_support::{budget, identity};

#[test]
fn opaque_reopen_rejects_missing_metadata_history_corrupt_payload_and_orphan_rows() {
    for fault in 0..9 {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("opaque");
        let mut store = open_opaque_record_repository(&path, identity(), budget()).unwrap();
        let initial = opaque_record_cursor(&store).unwrap();
        compare_and_append_opaque_records(&mut store, initial, &[vec![1], vec![2]]).unwrap();
        drop(store);
        let raw = rocksdb::DB::open_default(&path).unwrap();
        let first_key = [b"eve/opaque/v1/record/".as_slice(), &1_u64.to_be_bytes()].concat();
        let mut writes = rocksdb::WriteOptions::default();
        writes.set_sync(true);
        match fault {
            0 => raw.delete_opt(b"eve/opaque/v1/schema", &writes).unwrap(),
            1 => raw.delete_opt(b"eve/opaque/v1/identity", &writes).unwrap(),
            2 => raw.delete_opt(b"eve/opaque/v1/head", &writes).unwrap(),
            3 => raw.delete_opt(&first_key, &writes).unwrap(),
            4 => {
                let mut bytes = raw.get(&first_key).unwrap().unwrap();
                *bytes.last_mut().unwrap() ^= 1;
                raw.put_opt(&first_key, bytes, &writes).unwrap();
            }
            5 => raw
                .put_opt(
                    b"eve/opaque/v1/head",
                    [
                        initial.sequence.to_be_bytes().as_slice(),
                        &initial.content_hash,
                    ]
                    .concat(),
                    &writes,
                )
                .unwrap(),
            6 => raw
                .put_opt(b"foreign/key", b"must reject", &writes)
                .unwrap(),
            7 => {
                let orphan = [b"eve/opaque/v1/record/".as_slice(), &3_u64.to_be_bytes()].concat();
                raw.put_opt(orphan, raw.get(&first_key).unwrap().unwrap(), &writes)
                    .unwrap();
            }
            _ => raw.put_opt(&first_key, vec![0; 513], &writes).unwrap(),
        }
        drop(raw);
        assert!(
            open_opaque_record_repository(&path, identity(), budget()).is_err(),
            "fault {fault}"
        );
    }
}

#[test]
fn opaque_existing_database_with_all_metadata_removed_cannot_initialize_again() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("opaque");
    drop(open_opaque_record_repository(&path, identity(), budget()).unwrap());
    let raw = rocksdb::DB::open_default(&path).unwrap();
    for key in [
        b"eve/opaque/v1/schema".as_slice(),
        b"eve/opaque/v1/identity",
        b"eve/opaque/v1/head",
    ] {
        raw.delete(key).unwrap();
    }
    raw.flush_wal(true).unwrap();
    drop(raw);
    assert!(open_opaque_record_repository(&path, identity(), budget()).is_err());
}
