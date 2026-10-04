// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::execute_rpc::execute_rpc;
use crate::sync::applied::{
    finish_applied_state_service, poll_applied_durability, retained_tail_len,
    try_apply_recovery_bytes,
};
use std::{sync::Arc, time::Duration};

#[tokio::test]
async fn actual_storage_capacity_failure_reports_applied_and_durable_heights_without_claiming_ready()
 {
    let mut fixture = fixture(Some(1));
    try_apply_recovery_bytes(&mut fixture.owner, &fixture.chain.records[0]).unwrap();
    tokio::time::timeout(Duration::from_secs(5), async {
        while poll_applied_durability(&mut fixture.owner)
            .unwrap()
            .durable_recovery
            .0
            != 1
        {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    try_apply_recovery_bytes(&mut fixture.owner, &fixture.chain.records[1]).unwrap();
    tokio::time::timeout(Duration::from_secs(5), async {
        while poll_applied_durability(&mut fixture.owner).is_ok() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    let status = execute_rpc(Arc::clone(&fixture.context), "eve_getNodeStatus", vec![])
        .await
        .unwrap();
    assert_eq!(status["applied_height"], 2);
    assert_eq!(status["durable_height"], 1);
    assert_eq!(status["authenticated_height"], 2);
    assert_eq!(status["finalized_height"], 3);
    assert_eq!(status["storage_failed"], true);
    assert_eq!(status["ready"], false);
    assert_eq!(status["readiness_reason"], "storage failed");
    assert_eq!(status["durable_lag"], 1);
    assert_eq!(status["lag"], serde_json::Value::Null);
    let shutdown = finish_applied_state_service(fixture.owner);
    assert!(shutdown.repository.is_err());
    assert_eq!(retained_tail_len(&shutdown.unacknowledged_tail), 1);
}
