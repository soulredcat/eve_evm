// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{
    OpaqueRecordIdentity, compare_and_append_opaque_records, development_opaque_record_budget,
    opaque_record_cursor, open_opaque_record_repository, read_opaque_record,
    types::SimulatedOpaqueFailure,
};

#[test]
fn simulated_opaque_ambiguous_write_and_lost_ack_fence_until_real_reopen() {
    for after_sync in [false, true] {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("opaque");
        let identity = OpaqueRecordIdentity {
            genesis_hash: [1; 32],
            owner: [2; 32],
            domain: [3; 32],
        };
        let budget = development_opaque_record_budget();
        let mut store = open_opaque_record_repository(&path, identity, budget).unwrap();
        let initial = opaque_record_cursor(&store).unwrap();
        store.simulated_failure = Some(if after_sync {
            SimulatedOpaqueFailure::AfterSuccessfulSync
        } else {
            SimulatedOpaqueFailure::BeforeWrite
        });
        let error = compare_and_append_opaque_records(&mut store, initial, &[vec![9]]).unwrap_err();
        assert!(error.to_string().contains("SIMULATED"));
        assert!(opaque_record_cursor(&store).is_err());
        assert!(read_opaque_record(&store, 1).is_err());
        assert!(compare_and_append_opaque_records(&mut store, initial, &[vec![9]]).is_err());
        drop(store);
        let reopened = open_opaque_record_repository(&path, identity, budget).unwrap();
        assert_eq!(
            opaque_record_cursor(&reopened).unwrap().sequence,
            u64::from(after_sync)
        );
        assert_eq!(
            read_opaque_record(&reopened, 1).unwrap().is_some(),
            after_sync
        );
    }
}

#[test]
fn actual_missing_retained_row_fences_reads_and_future_appends() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("opaque");
    let identity = OpaqueRecordIdentity {
        genesis_hash: [1; 32],
        owner: [2; 32],
        domain: [3; 32],
    };
    let mut store =
        open_opaque_record_repository(&path, identity, development_opaque_record_budget()).unwrap();
    let initial = opaque_record_cursor(&store).unwrap();
    let ack = compare_and_append_opaque_records(&mut store, initial, &[vec![9]]).unwrap();
    let key = crate::records::encoding::opaque_record_key(1);
    store.database.delete(key).unwrap();
    assert!(read_opaque_record(&store, 1).is_err());
    assert!(opaque_record_cursor(&store).is_err());
    assert!(compare_and_append_opaque_records(&mut store, ack.store_head, &[vec![1]]).is_err());
}
