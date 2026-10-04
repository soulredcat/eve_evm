// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::execute_rpc::execute_rpc;
use crate::sync::applied::{
    capture_applied_state, finish_applied_state_service,
    tests::{import_fixtures::signed_transaction, pause_compact_append},
    try_apply_recovery_bytes,
};
use alloy_primitives::{U256, keccak256};
use serde_json::json;
use std::{sync::Arc, time::Duration};

#[tokio::test]
async fn actual_applied_ram_queries_proof_call_and_receipt_continue_while_storage_append_is_paused()
{
    let mut fixture = fixture(None);
    let (entered, resume) = pause_compact_append(&fixture.owner);
    try_apply_recovery_bytes(&mut fixture.owner, &fixture.chain.records[0]).unwrap();
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    fixture
        .context
        .pool
        .applied_committed(capture_applied_state(&fixture.reader).unwrap())
        .await
        .unwrap();
    let balance = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_getBalance",
        vec![json!(fixture.sender), json!("latest")],
    )
    .await;
    let storage = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_getStorageAt",
        vec![json!(fixture.contract), json!("0x0"), json!("latest")],
    )
    .await;
    let proof = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_getProof",
        vec![json!(fixture.contract), json!(["0x0"]), json!("latest")],
    )
    .await;
    let call = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_call",
        vec![
            json!({"from":fixture.sender,"to":fixture.contract}),
            json!("latest"),
        ],
    )
    .await;
    let block = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_getBlockByNumber",
        vec![json!("latest"), json!(false)],
    )
    .await;
    let receipt = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_getTransactionReceipt",
        vec![json!(keccak256(signed_transaction()))],
    )
    .await;
    let status = execute_rpc(Arc::clone(&fixture.context), "eve_getNodeStatus", vec![]).await;
    // Release the actual writer before assertions, including any RPC error.
    resume.send(()).unwrap();
    assert_eq!(
        balance.unwrap(),
        crate::rpc::encoding::quantity(
            fixture.chain.commits[1].state.accounts[&fixture.sender].balance
        )
    );
    assert_eq!(
        storage.unwrap(),
        json!(format!("0x{:064x}", U256::from(99)))
    );
    let proof = proof.unwrap();
    assert_eq!(proof["eveHeight"], "0x1");
    assert_eq!(proof["eveAuthenticatedHeight"], "0x1");
    assert_eq!(proof["eveVerificationMode"], "AUTHENTICATED_IMPORT");
    assert_eq!(call.unwrap(), json!("0x"));
    assert_eq!(block.unwrap()["number"], "0x1");
    assert_eq!(receipt.unwrap()["status"], "0x1");
    let status = status.unwrap();
    assert_eq!(status["applied_height"], 1);
    assert_eq!(status["durable_height"], 0);
    assert_eq!(status["authenticated_height"], 1);
    assert_eq!(status["finalized_height"], 2);
    assert_eq!(status["authenticated_finality"], true);
    assert_eq!(status["ready"], false);
    assert_eq!(status["lag"], serde_json::Value::Null);
    assert_eq!(fixture.context.bytes.available_permits(), 32 * 1024);
    drop(finish_applied_state_service(fixture.owner));
}
