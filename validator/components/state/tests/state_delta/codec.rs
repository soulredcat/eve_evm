// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{
    StateDeltaChunk, StateDeltaPayload, StateDeltaRequest, decode_state_delta_chunk,
    decode_state_delta_payload, decode_state_delta_request, development_state_budget,
    encode_state_delta_chunk, encode_state_delta_payload, encode_state_delta_request,
    hash_state_delta_bytes, measure_state_delta_payload, preflight_state_delta_payload,
    project_state_journal, state_delta_payload_stats,
};

#[test]
fn canonical_delta_request_payload_and_chunk_roundtrip_with_bound_count_stats() {
    let parent = super::support::genesis();
    let target = super::support::with_slots();
    let budget = development_state_budget();
    let request = StateDeltaRequest {
        parent: parent.target.clone(),
        target_height: 1,
        offset: 0,
        maximum_chunk_bytes: 128,
    };
    let request_bytes = encode_state_delta_request(&request).unwrap();
    assert_eq!(decode_state_delta_request(&request_bytes).unwrap(), request);
    let payload = StateDeltaPayload {
        journal: project_state_journal(&parent.state, &parent.target, &target.state, 1, &budget)
            .unwrap(),
        execution: target.block.clone(),
    };
    let bytes = encode_state_delta_payload(&payload, &budget).unwrap();
    assert_eq!(
        measure_state_delta_payload(&payload, &budget).unwrap(),
        bytes.len()
    );
    let preflight = preflight_state_delta_payload(&bytes, &budget).unwrap();
    assert_eq!(
        state_delta_payload_stats(&preflight).encoded_bytes,
        bytes.len()
    );
    assert_eq!(
        state_delta_payload_stats(&preflight)
            .journal
            .operation_count,
        payload.journal.operations.len()
    );
    assert_eq!(decode_state_delta_payload(&preflight).unwrap(), payload);
    let chunk = StateDeltaChunk {
        parent: parent.target,
        target: target.target.clone(),
        durable_tip: target.target,
        body_sha256: hash_state_delta_bytes(&bytes),
        total_length: bytes.len() as u64,
        offset: 0,
        data: bytes[..128].to_vec().into(),
    };
    assert_eq!(
        decode_state_delta_chunk(&encode_state_delta_chunk(&chunk).unwrap()).unwrap(),
        chunk
    );
}

#[test]
fn changed_chunk_bytes_or_header_fails_checksum_and_parent_is_exact() {
    let parent = super::support::genesis();
    let target = super::support::with_slots();
    let chunk = StateDeltaChunk {
        parent: parent.target,
        target: target.target.clone(),
        durable_tip: target.target,
        body_sha256: [0x31; 32],
        total_length: 2,
        offset: 0,
        data: vec![1, 2].into(),
    };
    let bytes = encode_state_delta_chunk(&chunk).unwrap();
    for index in [0, bytes.len() - 33, bytes.len() - 1] {
        let mut changed = bytes.clone();
        changed[index] ^= 1;
        assert!(matches!(
            decode_state_delta_chunk(&changed),
            Err(eve_state::StateDeltaError::ChecksumMismatch)
        ));
    }
    let mut request = StateDeltaRequest {
        parent: chunk.parent,
        target_height: 1,
        offset: 0,
        maximum_chunk_bytes: 1,
    };
    request.parent.content_digest.0[0] ^= 1;
    let decoded =
        decode_state_delta_request(&encode_state_delta_request(&request).unwrap()).unwrap();
    assert_eq!(decoded.parent, request.parent);
}

#[test]
fn every_truncated_request_and_payload_prefix_plus_trailing_data_reject() {
    let parent = super::support::genesis();
    let target = super::support::with_slots();
    let budget = development_state_budget();
    let request = encode_state_delta_request(&StateDeltaRequest {
        parent: parent.target.clone(),
        target_height: 1,
        offset: 0,
        maximum_chunk_bytes: 128,
    })
    .unwrap();
    for end in 0..request.len() {
        assert!(decode_state_delta_request(&request[..end]).is_err());
    }
    let payload = StateDeltaPayload {
        journal: project_state_journal(&parent.state, &parent.target, &target.state, 1, &budget)
            .unwrap(),
        execution: target.block,
    };
    let mut bytes = encode_state_delta_payload(&payload, &budget).unwrap();
    for end in 0..bytes.len() {
        assert!(preflight_state_delta_payload(&bytes[..end], &budget).is_err());
    }
    bytes.push(0);
    assert!(preflight_state_delta_payload(&bytes, &budget).is_err());
}
