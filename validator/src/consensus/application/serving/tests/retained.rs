// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{advance, request};
use crate::consensus::application::{
    serving::{development_delta_serving_budget, serve_state_delta_query},
    tests::fixture::test_application,
};
use eve_state::{
    apply_state_journal, decode_state_delta_chunk, decode_state_delta_payload,
    hash_state_delta_bytes, preflight_state_delta_payload,
};

#[test]
fn cached_and_historical_queries_are_exact_idempotent_and_reconstruct_retained_transition() {
    let mut fixture = test_application(false);
    let parent = fixture.genesis.clone();
    let first = advance(&mut fixture);
    let budget = development_delta_serving_budget();
    let query = request(&parent.target, 1, 0, 64);
    let one = serve_state_delta_query(&mut fixture.application, &query, budget);
    let duplicate = serve_state_delta_query(&mut fixture.application, &query, budget);
    assert_eq!(one.code, 0);
    assert_eq!(one.value, duplicate.value);
    let chunk = decode_state_delta_chunk(&one.value).unwrap();
    assert_eq!(chunk.parent, parent.target);
    assert_eq!(chunk.target, first.target);
    assert_eq!(chunk.data.len(), 64);
    let second = advance(&mut fixture);
    let later = serve_state_delta_query(&mut fixture.application, &query, budget);
    assert_eq!(later.code, 0);
    let later_chunk = decode_state_delta_chunk(&later.value).unwrap();
    assert_eq!(later_chunk.data, chunk.data);
    assert_eq!(later_chunk.body_sha256, chunk.body_sha256);
    assert_eq!(later_chunk.total_length, chunk.total_length);
    assert_eq!(later_chunk.target, first.target);
    assert_eq!(later_chunk.durable_tip, second.target);
    let all = serve_state_delta_query(
        &mut fixture.application,
        &request(&parent.target, 1, 0, 4_194_304),
        budget,
    );
    assert_eq!(all.code, 0);
    let all = decode_state_delta_chunk(&all.value).unwrap();
    assert_eq!(all.data.len() as u64, all.total_length);
    assert_eq!(hash_state_delta_bytes(&all.data), all.body_sha256);
    let decoded = decode_state_delta_payload(
        &preflight_state_delta_payload(&all.data, &fixture.application.config.logical_budget)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(decoded.execution, first.block);
    assert_eq!(
        apply_state_journal(
            &parent.state,
            &parent.target,
            &decoded.journal,
            &fixture.application.config.logical_budget
        )
        .unwrap(),
        first.state
    );
}

#[test]
fn nonzero_offset_returns_exact_remaining_requested_bytes_and_unchanged_identity() {
    let mut fixture = test_application(false);
    advance(&mut fixture);
    let parent = fixture.genesis.target.clone();
    let budget = development_delta_serving_budget();
    let full = serve_state_delta_query(
        &mut fixture.application,
        &request(&parent, 1, 0, 4_194_304),
        budget,
    );
    let full = decode_state_delta_chunk(&full.value).unwrap();
    let offset = full.total_length - 7;
    let tail = serve_state_delta_query(
        &mut fixture.application,
        &request(&parent, 1, offset, 64),
        budget,
    );
    assert_eq!(tail.code, 0);
    let tail = decode_state_delta_chunk(&tail.value).unwrap();
    assert_eq!(tail.data, full.data.slice(offset as usize..));
    assert_eq!(tail.data.len(), 7);
    assert_eq!(tail.body_sha256, full.body_sha256);
}
