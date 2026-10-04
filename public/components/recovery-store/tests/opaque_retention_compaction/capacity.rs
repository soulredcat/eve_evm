// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{authenticate_retained::authenticate_retained, fixtures};
use eve_storage::records::{
    compact_opaque_records, compare_and_append_opaque_records, opaque_record_cursor,
    open_opaque_record_repository, read_opaque_record,
};

#[test]
fn keep_all_capacity_refusal_and_actual_compaction_reopen_preserve_signed_recovery() {
    let fixture = fixtures::fixture();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("recovery");
    let mut repository =
        open_opaque_record_repository(&path, fixtures::identity(), fixtures::budget()).unwrap();
    let initial = opaque_record_cursor(&repository).unwrap();
    let synced =
        compare_and_append_opaque_records(&mut repository, initial, &fixture.wires).unwrap();
    let first = read_opaque_record(&repository, 1).unwrap().unwrap();
    let second = read_opaque_record(&repository, 2).unwrap().unwrap();
    assert!(
        compare_and_append_opaque_records(&mut repository, synced.store_head, &fixture.wires[..1])
            .is_err()
    );
    assert_eq!(
        opaque_record_cursor(&repository).unwrap(),
        synced.store_head
    );
    authenticate_retained(&repository, &fixture);
    let request = fixtures::request(&repository);
    let charge = fixtures::charge(&repository);
    assert_eq!(
        compact_opaque_records(&mut repository, &request, charge).unwrap(),
        synced.store_head
    );
    assert_eq!(read_opaque_record(&repository, 1).unwrap().unwrap(), first);
    assert_eq!(read_opaque_record(&repository, 2).unwrap().unwrap(), second);
    assert!(read_opaque_record(&repository, 3).unwrap().is_none());
    assert!(
        compare_and_append_opaque_records(&mut repository, synced.store_head, &fixture.wires[..1])
            .is_err()
    );
    drop(repository);
    let reopened =
        open_opaque_record_repository(&path, fixtures::identity(), fixtures::budget()).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), synced.store_head);
    assert_eq!(read_opaque_record(&reopened, 1).unwrap().unwrap(), first);
    assert_eq!(read_opaque_record(&reopened, 2).unwrap().unwrap(), second);
    authenticate_retained(&reopened, &fixture);
}
