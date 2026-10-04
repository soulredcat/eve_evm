// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_storage::records::{
    compact_opaque_records, compare_and_append_opaque_records, opaque_record_cursor,
    open_opaque_record_repository, read_opaque_record, required_opaque_compaction_reservation,
};

#[test]
fn maintenance_refuses_wrong_namespace_head_count_and_missing_read_reservation() {
    let fixture = fixtures::fixture();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("recovery");
    let mut repository =
        open_opaque_record_repository(&path, fixtures::identity(), fixtures::budget()).unwrap();
    let initial = opaque_record_cursor(&repository).unwrap();
    compare_and_append_opaque_records(&mut repository, initial, &fixture.wires).unwrap();
    let request = fixtures::request(&repository);
    let charge = fixtures::charge(&repository);
    let mut wrong = request;
    wrong.expected_identity.domain[0] ^= 1;
    assert!(compact_opaque_records(&mut repository, &wrong, charge).is_err());
    wrong = request;
    wrong.expected_head.content_hash[0] ^= 1;
    assert!(compact_opaque_records(&mut repository, &wrong, charge).is_err());
    for maximum_records in [0, 1, 3, u64::MAX] {
        wrong = request;
        wrong.maximum_records = maximum_records;
        assert!(compact_opaque_records(&mut repository, &wrong, charge).is_err());
    }
    assert!(compact_opaque_records(&mut repository, &request, charge - 1).is_err());
    assert_eq!(
        opaque_record_cursor(&repository).unwrap(),
        request.expected_head
    );
    assert_eq!(
        read_opaque_record(&repository, 1).unwrap().unwrap().payload,
        fixture.wires[0]
    );
    assert_eq!(
        read_opaque_record(&repository, 2).unwrap().unwrap().payload,
        fixture.wires[1]
    );
    assert!(
        open_opaque_record_repository(&path, fixtures::identity(), fixtures::budget()).is_err()
    );
    assert_eq!(
        compact_opaque_records(&mut repository, &request, charge).unwrap(),
        request.expected_head
    );
}

#[test]
fn empty_namespace_compaction_keeps_real_bootstrap_and_invalid_read_budget_rejects() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("recovery");
    let mut repository =
        open_opaque_record_repository(&path, fixtures::identity(), fixtures::budget()).unwrap();
    let request = fixtures::request(&repository);
    let charge = fixtures::charge(&repository);
    assert_eq!(request.expected_head.sequence, 0);
    assert_eq!(
        compact_opaque_records(&mut repository, &request, charge).unwrap(),
        request.expected_head
    );
    let mut overflowing = fixtures::budget();
    overflowing.maximum_read_bytes = usize::MAX;
    assert!(required_opaque_compaction_reservation(&overflowing).is_err());
    let mut invalid = fixtures::budget();
    invalid.maximum_read_bytes = 87;
    assert!(required_opaque_compaction_reservation(&invalid).is_err());
    drop(repository);
    let reopened =
        open_opaque_record_repository(&path, fixtures::identity(), fixtures::budget()).unwrap();
    assert_eq!(
        opaque_record_cursor(&reopened).unwrap(),
        request.expected_head
    );
}
