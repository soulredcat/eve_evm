// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::tests::import_fixtures::ImportChain;
use eve_state::{development_state_budget, encode_state_commit};
use eve_storage::records::{
    OpaqueRecordDisposition, OpaqueRecordIdentity, OpaqueRecordRepository,
    compare_and_append_opaque_records, development_opaque_record_budget, opaque_record_cursor,
    open_opaque_record_repository,
};
use std::path::Path;
/// Actual independently namespaced peer/validator-source WAL, with canonical commits and certified-input wire rows.
/// Availability assertions here are fixture-local and confer no finality or operator independence.
pub(super) fn store_peer_tail(
    path: &Path,
    chain: &ImportChain,
    tail: Option<&[u8]>,
) -> OpaqueRecordRepository {
    let identity = OpaqueRecordIdentity {
        genesis_hash: chain.commits[0].target.identity.genesis.0.0,
        owner: [0x71; 32],
        domain: [0x72; 32],
    };
    let mut budget = development_opaque_record_budget();
    budget.maximum_batch_records = 1;
    budget.maximum_open_files = 32;
    let mut repository = open_opaque_record_repository(path, identity, budget).unwrap();
    let logical = development_state_budget();
    let mut rows = vec![
        encode_state_commit(&chain.commits[1], &logical).unwrap(),
        chain.records[0].clone(),
    ];
    if let Some(tail) = tail {
        rows.push(encode_state_commit(&chain.commits[2], &logical).unwrap());
        rows.push(tail.to_vec());
    }
    for (index, payload) in rows.into_iter().enumerate() {
        let parent = opaque_record_cursor(&repository).unwrap();
        let ack = compare_and_append_opaque_records(&mut repository, parent, &[payload]).unwrap();
        assert_eq!(ack.disposition, OpaqueRecordDisposition::NewlySynced);
        assert_eq!(ack.appended.sequence, index as u64 + 1);
        assert_eq!(ack.store_head, ack.appended);
    }
    repository
}
