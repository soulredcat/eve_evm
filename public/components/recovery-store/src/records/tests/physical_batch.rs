// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{
    OpaqueRecordIdentity, compare_and_append_opaque_records, development_opaque_record_budget,
    opaque_record_cursor, open_opaque_record_repository,
};

#[test]
fn opaque_physical_batch_limit_includes_head_and_exact_replays() {
    let identity = OpaqueRecordIdentity {
        genesis_hash: [1; 32],
        owner: [2; 32],
        domain: [3; 32],
    };
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("opaque");
    let mut budget = development_opaque_record_budget();
    budget.maximum_record_bytes = 512;
    budget.maximum_read_bytes = 512;
    let mut store = open_opaque_record_repository(&path, identity, budget).unwrap();
    let expected = opaque_record_cursor(&store).unwrap();
    let payloads = vec![vec![4; 424]];
    let (batch, _) =
        crate::records::repository::append::build_opaque_batch(&store, expected, &payloads)
            .unwrap();
    let physical = batch.size_in_bytes();
    assert!(
        physical > 512 + 40,
        "Rocks framing and durable-head metadata must both count"
    );
    store.budget.maximum_batch_bytes = physical - 1;
    assert!(compare_and_append_opaque_records(&mut store, expected, &payloads).is_err());
    assert_eq!(opaque_record_cursor(&store).unwrap(), expected);
    store.budget.maximum_batch_bytes = physical;
    let ack = compare_and_append_opaque_records(&mut store, expected, &payloads).unwrap();
    store.budget.maximum_batch_bytes = physical - 1;
    assert!(compare_and_append_opaque_records(&mut store, expected, &payloads).is_err());
    assert_eq!(opaque_record_cursor(&store).unwrap(), ack.store_head);
}
