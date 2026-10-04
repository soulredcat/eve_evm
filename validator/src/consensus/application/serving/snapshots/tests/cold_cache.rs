// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{development_snapshot_serving_budget, serve_checkpoint_query};
use super::fixtures::request;
use crate::consensus::application::{
    serving::tests::fixtures::advance, tests::fixture::test_application,
};
use eve_storage::{
    checkpoints::messages::CheckpointRequestKind,
    state::{create_state_service, read_cached_state_service, state_reader},
};
#[test]
fn cold_cache_refuses_configured_decode_ceiling_before_loading_database_state() {
    let mut fixture = test_application(false);
    advance(&mut fixture);
    fixture.application.service = std::sync::Arc::new(create_state_service(state_reader(
        &fixture.application.repository,
    )));
    assert!(
        read_cached_state_service(&fixture.application.service)
            .unwrap()
            .is_none()
    );
    let query = request(&fixture.genesis.target, 1, CheckpointRequestKind::Execution);
    assert_eq!(
        serve_checkpoint_query(
            &mut fixture.application,
            &query,
            development_snapshot_serving_budget()
        )
        .code,
        4
    );
    assert!(
        read_cached_state_service(&fixture.application.service)
            .unwrap()
            .is_none()
    );
}
