// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{development_snapshot_serving_budget, serve_checkpoint_query};
use super::fixtures::{decode, request};
use crate::consensus::application::{
    serving::tests::fixtures::advance, tests::fixture::test_application,
};
use eve_storage::checkpoints::messages::{CheckpointRequestKind, CheckpointResponse};
#[test]
fn wrong_network_missing_height_manifest_index_and_fixed_request_errors_have_stable_codes() {
    let mut fixture = test_application(false);
    advance(&mut fixture);
    let budget = development_snapshot_serving_budget();
    let mut query = request(&fixture.genesis.target, 1, CheckpointRequestKind::Execution);
    let mut wrong = fixture.genesis.target.clone();
    wrong.identity.network_name.push('x');
    let wrong = request(&wrong, 1, CheckpointRequestKind::Execution);
    assert_eq!(
        serve_checkpoint_query(&mut fixture.application, &wrong, budget).code,
        1
    );
    let future = request(&fixture.genesis.target, 2, CheckpointRequestKind::Execution);
    assert_eq!(
        serve_checkpoint_query(&mut fixture.application, &future, budget).code,
        3
    );
    let chunk = request(
        &fixture.genesis.target,
        1,
        CheckpointRequestKind::Chunk {
            chunk_bytes: 64,
            manifest_id: [0; 32],
            index: 0,
        },
    );
    assert_eq!(
        serve_checkpoint_query(&mut fixture.application, &chunk, budget).code,
        3
    );
    let manifest_query = request(
        &fixture.genesis.target,
        1,
        CheckpointRequestKind::Manifest { chunk_bytes: 64 },
    );
    let manifest_response =
        serve_checkpoint_query(&mut fixture.application, &manifest_query, budget);
    let CheckpointResponse::Manifest { manifest_id, .. } =
        decode(&fixture.application, &manifest_response)
    else {
        panic!("wrong response kind")
    };
    let missing = request(
        &fixture.genesis.target,
        1,
        CheckpointRequestKind::Chunk {
            chunk_bytes: 64,
            manifest_id,
            index: 4_095,
        },
    );
    assert_eq!(
        serve_checkpoint_query(&mut fixture.application, &missing, budget).code,
        3
    );
    query.prove = true;
    assert_eq!(
        serve_checkpoint_query(&mut fixture.application, &query, budget).code,
        6
    );
    query.prove = false;
    let offset = b"EVE_CHECKPOINT_REQUEST_V1".len();
    query.data[offset] = 2;
    assert_eq!(
        serve_checkpoint_query(&mut fixture.application, &query, budget).code,
        2
    );
    query.data[offset] = 1;
    query.data[offset + 1] = 1;
    assert_eq!(
        serve_checkpoint_query(&mut fixture.application, &query, budget).code,
        2
    );
    fixture.application.fenced = true;
    assert_eq!(
        serve_checkpoint_query(&mut fixture.application, &chunk, budget).code,
        5
    );
}
#[test]
fn declared_working_chunk_manifest_and_request_limits_refuse_without_state_mutation() {
    let mut fixture = test_application(false);
    let first = advance(&mut fixture);
    let budget = development_snapshot_serving_budget();
    let query = request(
        &fixture.genesis.target,
        1,
        CheckpointRequestKind::Manifest { chunk_bytes: 64 },
    );
    for changed in [
        super::super::SnapshotServingBudget {
            maximum_working_bytes: 1,
            ..budget
        },
        super::super::SnapshotServingBudget {
            maximum_working_bytes: 64 * 1_048_576 + 1,
            ..budget
        },
        super::super::SnapshotServingBudget {
            maximum_manifest_bytes: 75,
            ..budget
        },
        super::super::SnapshotServingBudget {
            maximum_request_bytes: 1,
            ..budget
        },
        super::super::SnapshotServingBudget {
            maximum_chunk_bytes: 32,
            ..budget
        },
    ] {
        assert_eq!(
            serve_checkpoint_query(&mut fixture.application, &query, changed).code,
            4
        );
    }
    assert_eq!(
        eve_storage::state::read_state_service(&fixture.application.service)
            .unwrap()
            .commit(),
        &first
    );
}
