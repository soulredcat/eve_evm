// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::execute_rpc::execute_rpc;
use crate::sync::applied::{finish_applied_state_service, try_apply_recovery_bytes};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn applied_history_absence_is_explicit_gap_or_not_ready_instead_of_null_or_pruned() {
    let mut fixture = fixture(None);
    try_apply_recovery_bytes(&mut fixture.owner, &fixture.chain.records[0]).unwrap();
    let shutdown = finish_applied_state_service(fixture.owner);
    for (method, params, prefix) in [
        (
            "eth_getBalance",
            vec![json!(fixture.sender), json!("0x0")],
            "GAP:",
        ),
        (
            "eth_getBalance",
            vec![json!(fixture.sender), json!("0x2")],
            "NOT_READY:",
        ),
        (
            "eth_getBalance",
            vec![json!(fixture.sender), json!("pending")],
            "NOT_READY:",
        ),
        (
            "eth_getBlockByHash",
            vec![json!(format!("0x{}", "11".repeat(32))), json!(false)],
            "GAP:",
        ),
        (
            "eth_getTransactionReceipt",
            vec![json!(format!("0x{}", "22".repeat(32)))],
            "GAP:",
        ),
        (
            "eth_getTransactionByHash",
            vec![json!(format!("0x{}", "33".repeat(32)))],
            "GAP:",
        ),
        (
            "eth_getLogs",
            vec![json!({"fromBlock":"0x0","toBlock":"latest"})],
            "GAP:",
        ),
        (
            "eth_feeHistory",
            vec![json!("0x2"), json!("latest")],
            "GAP:",
        ),
    ] {
        let failure = execute_rpc(Arc::clone(&fixture.context), method, params)
            .await
            .unwrap_err();
        assert_eq!(failure.code(), -32001);
        assert!(
            failure.message().starts_with(prefix),
            "{method}: {}",
            failure.message()
        );
        assert!(!failure.message().contains("PRUNED"));
    }
    assert_eq!(
        execute_rpc(
            Arc::clone(&fixture.context),
            "eth_getBlockByNumber",
            vec![json!("finalized"), json!(false)]
        )
        .await
        .unwrap()["number"],
        "0x1"
    );
    let fees = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_feeHistory",
        vec![json!("0x1"), json!("latest")],
    )
    .await
    .unwrap();
    assert_eq!(fees["oldestBlock"], "0x1");
    assert_eq!(fees["baseFeePerGas"].as_array().unwrap().len(), 2);
    drop(shutdown);
}
