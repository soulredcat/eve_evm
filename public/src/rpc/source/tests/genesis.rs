// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::{durable_rpc_source, execute_rpc::execute_rpc};
use crate::sync::applied::finish_applied_state_service;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn applied_genesis_reports_local_trust_unknown_freshness_and_no_fabricated_durable_source() {
    let fixture = fixture(None);
    assert!(
        durable_rpc_source(&fixture.context)
            .err()
            .unwrap()
            .message()
            .starts_with("GAP:")
    );
    assert_eq!(fixture.context.producer_bytes.num_permits(), 0);
    assert_eq!(fixture.context.bytes.available_permits(), 32 * 1024);
    let status = execute_rpc(Arc::clone(&fixture.context), "eve_getNodeStatus", vec![])
        .await
        .unwrap();
    assert_eq!(status["applied_height"], 0);
    assert_eq!(status["durable_height"], 0);
    assert_eq!(status["finalized_height"], 0);
    assert_eq!(status["authenticated_height"], serde_json::Value::Null);
    assert_eq!(status["verification_mode"], "LOCAL_GENESIS_TRUSTED");
    assert_eq!(status["authenticated_finality"], false);
    assert_eq!(status["ready"], false);
    assert_eq!(status["peer_count"], serde_json::Value::Null);
    assert_eq!(status["lag"], serde_json::Value::Null);
    assert_eq!(status["database_sequence"], serde_json::Value::Null);
    assert_eq!(status["readiness_reason"], "head freshness unknown");
    let certified = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_getBalance",
        vec![json!(fixture.sender), json!("finalized")],
    )
    .await
    .unwrap_err();
    assert!(certified.message().starts_with("NOT_READY:"));
    assert_eq!(
        execute_rpc(Arc::clone(&fixture.context), "eth_chainId", vec![])
            .await
            .unwrap(),
        json!("0x7a69")
    );
    assert_eq!(
        execute_rpc(Arc::clone(&fixture.context), "eth_blockNumber", vec![])
            .await
            .unwrap(),
        json!("0x0")
    );
    drop(finish_applied_state_service(fixture.owner));
}
