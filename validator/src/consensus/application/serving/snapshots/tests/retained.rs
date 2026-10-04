// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{development_snapshot_serving_budget, serve_checkpoint_query};
use super::fixtures::{decode, request};
use crate::consensus::application::{
    serving::tests::fixtures::advance, tests::fixture::test_application,
};
use eve_state::{decode_state_commit, encode_state_commit};
use eve_storage::checkpoints::messages::{CheckpointRequestKind, CheckpointResponse};
#[test]
fn cached_and_historical_snapshot_manifest_chunks_and_execution_share_exact_target() {
    let mut fixture = test_application(false);
    let first = advance(&mut fixture);
    let budget = development_snapshot_serving_budget();
    let query = request(
        &fixture.genesis.target,
        1,
        CheckpointRequestKind::Manifest { chunk_bytes: 64 },
    );
    let one = serve_checkpoint_query(&mut fixture.application, &query, budget);
    let duplicate = serve_checkpoint_query(&mut fixture.application, &query, budget);
    assert_eq!(one.value, duplicate.value);
    let CheckpointResponse::Manifest {
        target,
        durable_tip,
        manifest_id,
        manifest,
    } = decode(&fixture.application, &one)
    else {
        panic!("wrong response kind")
    };
    assert_eq!(target, first.target);
    assert_eq!(*durable_tip, first.target);
    let body = encode_state_commit(&first, &fixture.application.config.logical_budget).unwrap();
    let mut assembled = Vec::new();
    for index in 0..body.len().div_ceil(64) {
        let query = request(
            &fixture.genesis.target,
            1,
            CheckpointRequestKind::Chunk {
                chunk_bytes: 64,
                manifest_id,
                index: index as u32,
            },
        );
        let response = serve_checkpoint_query(&mut fixture.application, &query, budget);
        let CheckpointResponse::Chunk {
            target,
            data,
            manifest_id: actual,
            total_length,
            ..
        } = decode(&fixture.application, &response)
        else {
            panic!("wrong response kind")
        };
        assert_eq!(target, first.target);
        assert_eq!(actual, manifest_id);
        assert_eq!(total_length as usize, body.len());
        assembled.extend_from_slice(&data);
    }
    assert_eq!(assembled, body);
    assert_eq!(
        decode_state_commit(&assembled, &fixture.application.config.logical_budget).unwrap(),
        first
    );
    let second = advance(&mut fixture);
    let later = serve_checkpoint_query(&mut fixture.application, &query, budget);
    let CheckpointResponse::Manifest {
        target,
        durable_tip,
        manifest_id: later_id,
        manifest: later_manifest,
    } = decode(&fixture.application, &later)
    else {
        panic!("wrong response kind")
    };
    assert_eq!(target, first.target);
    assert_eq!(*durable_tip, second.target);
    assert_eq!(later_id, manifest_id);
    assert_eq!(later_manifest, manifest);
    let execution = request(&fixture.genesis.target, 1, CheckpointRequestKind::Execution);
    let response = serve_checkpoint_query(&mut fixture.application, &execution, budget);
    let CheckpointResponse::Execution { target, block } = decode(&fixture.application, &response)
    else {
        panic!("wrong response kind")
    };
    assert_eq!(target, first.target);
    assert_eq!(*block, first.block);
}
