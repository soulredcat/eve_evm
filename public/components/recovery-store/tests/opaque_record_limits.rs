// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod opaque_record_support;
use eve_storage::records::{
    compare_and_append_opaque_records, opaque_record_cursor, open_opaque_record_repository,
    read_opaque_record, validate_opaque_record_budget,
};
use opaque_record_support::{budget, identity};

#[test]
fn opaque_encoded_record_and_count_limits_reject_before_durable_mutation() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("opaque");
    let mut store = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    let initial = opaque_record_cursor(&store).unwrap();
    assert!(compare_and_append_opaque_records(&mut store, initial, &[vec![1; 425]]).is_err());
    assert!(compare_and_append_opaque_records(&mut store, initial, &vec![vec![1]; 9]).is_err());
    assert_eq!(opaque_record_cursor(&store).unwrap(), initial);
    let ack = compare_and_append_opaque_records(&mut store, initial, &[vec![1; 424]]).unwrap();
    assert_eq!(
        read_opaque_record(&store, 1)
            .unwrap()
            .unwrap()
            .payload
            .len(),
        424
    );
    assert_eq!(ack.store_head.sequence, 1);
}

#[test]
fn opaque_history_capacity_never_prunes_and_exact_replay_remains_available() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("opaque");
    let mut limits = budget();
    limits.maximum_retained_records = 2;
    let mut store = open_opaque_record_repository(&path, identity(), limits).unwrap();
    let initial = opaque_record_cursor(&store).unwrap();
    let payloads = vec![vec![1], vec![2]];
    let ack = compare_and_append_opaque_records(&mut store, initial, &payloads).unwrap();
    assert!(compare_and_append_opaque_records(&mut store, ack.store_head, &[vec![3]]).is_err());
    assert_eq!(opaque_record_cursor(&store).unwrap(), ack.store_head);
    assert_eq!(
        compare_and_append_opaque_records(&mut store, initial, &payloads)
            .unwrap()
            .store_head,
        ack.store_head
    );
    drop(store);
    limits.maximum_retained_records = 1;
    assert!(open_opaque_record_repository(&path, identity(), limits).is_err());
    let reopened = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), ack.store_head);
}

#[test]
fn opaque_invalid_resource_and_read_budgets_fail_closed() {
    for field in 0..7 {
        let mut limits = budget();
        match field {
            0 => limits.maximum_record_bytes = 87,
            1 => limits.maximum_read_bytes = 511,
            2 => limits.maximum_batch_bytes = 511,
            3 => limits.maximum_batch_records = 0,
            4 => limits.maximum_retained_records = 0,
            5 => limits.write_buffer_count = 0,
            _ => limits.maximum_retained_records = u64::MAX,
        }
        assert!(validate_opaque_record_budget(&limits).is_err());
    }
}
