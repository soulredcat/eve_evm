// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{StateBudget, StateCommit, development_state_budget, encode_state_commit};

pub fn fixture() -> (StateCommit, Vec<u8>, StateBudget) {
    let mut commit = crate::support::with_slots();
    let mut state = commit.state.clone();
    let unused = eve_state::Bytes::from_static(&[0x61, 0x01, 0x01, 0x00]);
    state
        .codes
        .insert(alloy_primitives::keccak256(&unused), unused);
    let budget = development_state_budget();
    commit = eve_state::build_state_commit(commit.parent, state, commit.block, &budget).unwrap();
    let bytes = encode_state_commit(&commit, &budget).unwrap();
    (commit, bytes, budget)
}

/// Test-only byte topology mutation; no alternate production codec or validity claim.
pub fn fields(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut remaining = bytes;
    let mut body = alloy_rlp::Header::decode_bytes(&mut remaining, true).unwrap();
    assert!(remaining.is_empty());
    let mut fields = Vec::new();
    while !body.is_empty() {
        let before = body;
        let header = alloy_rlp::Header::decode(&mut body).unwrap();
        body = &body[header.payload_length..];
        fields.push(before[..before.len() - body.len()].to_vec());
    }
    fields
}

pub fn list(fields: &[Vec<u8>]) -> Vec<u8> {
    let length = fields.iter().map(Vec::len).sum();
    let mut output = Vec::new();
    alloy_rlp::Header {
        list: true,
        payload_length: length,
    }
    .encode(&mut output);
    for field in fields {
        output.extend_from_slice(field);
    }
    output
}

pub fn replace_state_map(bytes: &[u8], field: usize, values: Vec<Vec<u8>>) -> Vec<u8> {
    let mut commit = fields(bytes);
    let mut state = fields(&commit[3]);
    state[field] = list(&values);
    commit[3] = list(&state);
    list(&commit)
}
