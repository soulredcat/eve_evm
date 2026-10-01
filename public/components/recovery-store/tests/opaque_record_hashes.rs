// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod opaque_record_support;
use eve_storage::records::{
    compare_and_append_opaque_records, opaque_record_cursor, open_opaque_record_repository,
};
use opaque_record_support::{budget, identity};

#[test]
fn opaque_content_binding_matches_independent_node_sha256_vectors_and_full_namespace() {
    // Independently computed with pinned Node 24.21.0 SHA-256: fixed domain, identity,
    // parent sequence/hash, target sequence, payload length (u64 BE), exact payload.
    let expected_bootstrap = [
        218, 159, 224, 102, 66, 244, 119, 42, 51, 71, 197, 155, 235, 43, 172, 131, 55, 140, 83,
        119, 56, 4, 171, 19, 172, 139, 151, 140, 248, 208, 210, 182,
    ];
    let expected_record = [
        105, 59, 214, 51, 199, 72, 19, 48, 252, 71, 213, 240, 212, 40, 221, 44, 38, 207, 210, 185,
        218, 138, 18, 98, 244, 46, 95, 243, 81, 255, 162, 25,
    ];
    let temporary = tempfile::tempdir().unwrap();
    for field in 0..4 {
        let mut namespace = identity();
        match field {
            1 => namespace.genesis_hash[31] ^= 1,
            2 => namespace.owner[31] ^= 1,
            3 => namespace.domain[31] ^= 1,
            _ => {}
        }
        let mut store = open_opaque_record_repository(
            &temporary.path().join(format!("opaque-{field}")),
            namespace,
            budget(),
        )
        .unwrap();
        let initial = opaque_record_cursor(&store).unwrap();
        let ack =
            compare_and_append_opaque_records(&mut store, initial, &[b"opaque fixture".to_vec()])
                .unwrap();
        if field == 0 {
            assert_eq!(initial.content_hash, expected_bootstrap);
            assert_eq!(ack.appended.content_hash, expected_record);
            drop(store);
            let raw = rocksdb::DB::open_default(temporary.path().join("opaque-0")).unwrap();
            let expected_bytes = [
                1_u64.to_be_bytes().as_slice(),
                0_u64.to_be_bytes().as_slice(),
                &expected_bootstrap,
                &expected_record,
                14_u64.to_be_bytes().as_slice(),
                b"opaque fixture",
            ]
            .concat();
            let key = [b"eve/opaque/v1/record/".as_slice(), &1_u64.to_be_bytes()].concat();
            assert_eq!(raw.get(key).unwrap().unwrap(), expected_bytes);
        } else {
            assert_ne!(initial.content_hash, expected_bootstrap);
            assert_ne!(ack.appended.content_hash, expected_record);
        }
    }
}
