// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#![cfg(test)]

mod opaque_record_support;
use eve_storage::records::{
    OpaqueRecordCursor, compare_and_append_opaque_records, opaque_record_cursor,
    open_opaque_record_repository, prospective_opaque_record_cursor, read_opaque_record,
};
use opaque_record_support::{budget, identity};

#[test]
fn prospective_cursor_matches_independent_sha256_vector_without_advancing_store() {
    let directory = tempfile::tempdir().unwrap();
    let store =
        open_opaque_record_repository(&directory.path().join("opaque"), identity(), budget())
            .unwrap();
    let parent = opaque_record_cursor(&store).unwrap();
    let prospective = prospective_opaque_record_cursor(
        identity(),
        parent,
        b"opaque fixture",
        budget().maximum_record_bytes,
    )
    .unwrap();
    // Preserved independent pinned Node SHA-256 vector from opaque_record_hashes.
    assert_eq!(
        prospective,
        OpaqueRecordCursor {
            sequence: 1,
            content_hash: [
                105, 59, 214, 51, 199, 72, 19, 48, 252, 71, 213, 240, 212, 40, 221, 44, 38, 207,
                210, 185, 218, 138, 18, 98, 244, 46, 95, 243, 81, 255, 162, 25,
            ],
        }
    );
    assert_eq!(opaque_record_cursor(&store).unwrap(), parent);
    assert!(read_opaque_record(&store, 1).unwrap().is_none());
}

#[test]
fn prospective_record_chain_matches_synced_append_and_reopen() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("opaque");
    let mut store = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    let initial = opaque_record_cursor(&store).unwrap();
    let payloads = [
        b"record-one".to_vec(),
        b"record-two".to_vec(),
        b"record-three".to_vec(),
    ];
    let mut prospective = initial;
    for payload in &payloads {
        prospective = prospective_opaque_record_cursor(
            identity(),
            prospective,
            payload,
            budget().maximum_record_bytes,
        )
        .unwrap();
    }
    assert_eq!(opaque_record_cursor(&store).unwrap(), initial);
    let ack = compare_and_append_opaque_records(&mut store, initial, &payloads).unwrap();
    assert_eq!(ack.appended, prospective);
    assert_eq!(ack.store_head, prospective);
    drop(store);
    let reopened = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), prospective);
    for (index, payload) in payloads.iter().enumerate() {
        let sequence = u64::try_from(index + 1).unwrap();
        assert_eq!(
            read_opaque_record(&reopened, sequence)
                .unwrap()
                .unwrap()
                .payload,
            *payload
        );
    }
}

#[test]
fn prospective_cursor_binds_full_namespace_parent_and_payload_without_parent_authority() {
    let parent = OpaqueRecordCursor {
        sequence: 9,
        content_hash: [7; 32],
    };
    let namespace = identity();
    let maximum = budget().maximum_record_bytes;
    // An unretained hypothetical parent is accepted; derivation does not authenticate it.
    let expected =
        prospective_opaque_record_cursor(namespace, parent, b"payload", maximum).unwrap();
    assert_eq!(expected.sequence, 10);
    for field in 0..3 {
        let mut changed = namespace;
        match field {
            0 => changed.genesis_hash[31] ^= 1,
            1 => changed.owner[31] ^= 1,
            _ => changed.domain[31] ^= 1,
        }
        assert_ne!(
            prospective_opaque_record_cursor(changed, parent, b"payload", maximum).unwrap(),
            expected
        );
    }
    let changed_parent = OpaqueRecordCursor {
        content_hash: [8; 32],
        ..parent
    };
    assert_ne!(
        prospective_opaque_record_cursor(namespace, changed_parent, b"payload", maximum).unwrap(),
        expected
    );
    assert_ne!(
        prospective_opaque_record_cursor(namespace, parent, b"payloae", maximum).unwrap(),
        expected
    );
}

#[test]
fn prospective_cursor_checks_encoded_record_boundary_and_sequence_overflow() {
    let parent = OpaqueRecordCursor::default();
    let maximum = budget().maximum_record_bytes;
    // The independent local schema contract has 88 bytes of record metadata.
    let fitting = vec![0; maximum - 88];
    let oversized = vec![0; maximum - 87];
    assert!(prospective_opaque_record_cursor(identity(), parent, &fitting, maximum).is_ok());
    assert!(prospective_opaque_record_cursor(identity(), parent, &oversized, maximum).is_err());
    assert!(prospective_opaque_record_cursor(identity(), parent, b"", 88).is_ok());
    assert!(prospective_opaque_record_cursor(identity(), parent, b"", 87).is_err());
    let exhausted = OpaqueRecordCursor {
        sequence: u64::MAX,
        ..parent
    };
    assert!(prospective_opaque_record_cursor(identity(), exhausted, b"", maximum).is_err());
}
