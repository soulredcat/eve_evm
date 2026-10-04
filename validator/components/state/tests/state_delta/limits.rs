// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{
    MAXIMUM_STATE_DELTA_CHUNK_BYTES, MAXIMUM_STATE_DELTA_PAYLOAD_BYTES, StateDeltaChunk,
    StateDeltaError, StateDeltaPayload, StateDeltaRequest, decode_state_delta_request,
    development_state_budget, encode_state_delta_chunk, encode_state_delta_payload,
    encode_state_delta_request, measure_state_delta_payload, preflight_state_delta_payload,
    project_state_journal,
};

#[test]
fn request_versions_chunk_limit_and_declared_length_overflow_reject() {
    let parent = super::support::genesis();
    let mut request = StateDeltaRequest {
        parent: parent.target,
        target_height: 1,
        offset: 0,
        maximum_chunk_bytes: MAXIMUM_STATE_DELTA_CHUNK_BYTES as u32,
    };
    let mut bytes = encode_state_delta_request(&request).unwrap();
    assert!(decode_state_delta_request(&bytes).is_ok());
    bytes[15] = b'2';
    assert_eq!(
        decode_state_delta_request(&bytes),
        Err(StateDeltaError::UnsupportedVersion)
    );
    request.maximum_chunk_bytes += 1;
    assert_eq!(
        encode_state_delta_request(&request),
        Err(StateDeltaError::BudgetExceeded)
    );
    let mut overflow = b"EVE_DELTA_REQ_V1".to_vec();
    overflow.extend_from_slice(&u32::MAX.to_be_bytes());
    assert_eq!(
        decode_state_delta_request(&overflow),
        Err(StateDeltaError::BudgetExceeded)
    );
}

#[test]
fn chunk_offset_overflow_empty_data_and_beyond_body_are_rejected() {
    let parent = super::support::genesis();
    let target = super::support::with_slots();
    let mut chunk = StateDeltaChunk {
        parent: parent.target,
        target: target.target.clone(),
        durable_tip: target.target,
        body_sha256: [0x31; 32],
        total_length: 1,
        offset: u64::MAX,
        data: vec![1].into(),
    };
    assert_eq!(
        encode_state_delta_chunk(&chunk),
        Err(StateDeltaError::BudgetExceeded)
    );
    chunk.offset = 1;
    assert_eq!(
        encode_state_delta_chunk(&chunk),
        Err(StateDeltaError::BudgetExceeded)
    );
    chunk.offset = 0;
    chunk.data = Vec::new().into();
    assert_eq!(
        encode_state_delta_chunk(&chunk),
        Err(StateDeltaError::BudgetExceeded)
    );
}

#[test]
fn local_journal_and_execution_admission_is_checked_before_decoding() {
    assert_eq!(
        MAXIMUM_STATE_DELTA_PAYLOAD_BYTES,
        18 + 8 + 8_388_608 + 8_404_140
    );
    let parent = super::support::genesis();
    let target = super::support::with_slots();
    let budget = development_state_budget();
    let payload = StateDeltaPayload {
        journal: project_state_journal(&parent.state, &parent.target, &target.state, 1, &budget)
            .unwrap(),
        execution: target.block,
    };
    let bytes = encode_state_delta_payload(&payload, &budget).unwrap();
    for field in 0..2 {
        let mut restricted = budget;
        if field == 0 {
            restricted.maximum_journal_bytes = 1;
        } else {
            restricted.maximum_commit_bytes = 1;
        }
        assert!(preflight_state_delta_payload(&bytes, &restricted).is_err());
        assert!(measure_state_delta_payload(&payload, &restricted).is_err());
    }
}
