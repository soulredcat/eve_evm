// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{advance, request};
use crate::consensus::application::{
    serving::{development_delta_serving_budget, serve_state_delta_query},
    tests::fixture::test_application,
};
use eve_consensus_comet::wire::tendermint::abci::RequestQuery;
use eve_storage::state::read_state_service;

#[test]
fn wrong_network_exact_parent_and_missing_height_have_stable_failure_categories() {
    let mut fixture = test_application(false);
    let parent = fixture.genesis.target.clone();
    advance(&mut fixture);
    let budget = development_delta_serving_budget();
    let before = read_state_service(&fixture.application.service)
        .unwrap()
        .commit()
        .target
        .clone();
    let mut wrong_network = parent.clone();
    wrong_network.identity.genesis.0.0[0] ^= 1;
    let response = serve_state_delta_query(
        &mut fixture.application,
        &request(&wrong_network, 1, 0, 64),
        budget,
    );
    assert_eq!((response.code, response.log.as_str()), (1, "WRONG_NETWORK"));
    let mut wrong_parent = parent.clone();
    wrong_parent.content_digest.0[0] ^= 1;
    let response = serve_state_delta_query(
        &mut fixture.application,
        &request(&wrong_parent, 1, 0, 64),
        budget,
    );
    assert_eq!((response.code, response.log.as_str()), (3, "GAP"));
    for height in [0, 2, u64::MAX] {
        let response = serve_state_delta_query(
            &mut fixture.application,
            &request(&parent, height, 0, 64),
            budget,
        );
        assert_eq!(response.log, "GAP");
    }
    assert_eq!(
        read_state_service(&fixture.application.service)
            .unwrap()
            .commit()
            .target,
        before
    );
}

#[test]
fn unsupported_malformed_fenced_and_tight_operator_limits_fail_without_writes() {
    let mut fixture = test_application(false);
    let parent = fixture.genesis.target.clone();
    advance(&mut fixture);
    let mut budget = development_delta_serving_budget();
    let mut query = request(&parent, 1, 0, 64);
    query.path = "/eve/recovery/v2/delta".into();
    assert_eq!(
        serve_state_delta_query(&mut fixture.application, &query, budget).log,
        "UNSUPPORTED_VERSION"
    );
    query = RequestQuery {
        path: "/eve/recovery/v1/delta".into(),
        data: vec![0x11],
        ..Default::default()
    };
    assert_eq!(
        serve_state_delta_query(&mut fixture.application, &query, budget).log,
        "MALFORMED_REQUEST"
    );
    query = request(&parent, 1, 0, 64);
    budget.maximum_working_bytes = 1;
    assert_eq!(
        serve_state_delta_query(&mut fixture.application, &query, budget).log,
        "RESOURCE_LIMIT"
    );
    budget = development_delta_serving_budget();
    budget.maximum_chunk_bytes = 63;
    assert_eq!(
        serve_state_delta_query(&mut fixture.application, &query, budget).log,
        "RESOURCE_LIMIT"
    );
    budget = development_delta_serving_budget();
    fixture.application.fenced = true;
    assert_eq!(
        serve_state_delta_query(&mut fixture.application, &query, budget).log,
        "NOT_READY"
    );
}
