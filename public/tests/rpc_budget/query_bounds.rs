// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::execute_rpc::execute_rpc;
use alloy_primitives::Bytes;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn ta05_proof_slot_simulation_gas_calldata_and_log_range_bounds_are_enforced() {
    let fixture = fixture(Bytes::new());
    let context = fixture.context;
    let slots: Vec<_> = (0..256).map(|slot| json!(format!("0x{slot:x}"))).collect();
    let accepted = execute_rpc(
        Arc::clone(&context),
        "eth_getProof",
        vec![json!(fixture.contract), json!(slots), json!("latest")],
    )
    .await
    .unwrap();
    assert_eq!(accepted["storageProof"].as_array().unwrap().len(), 256);
    let too_many: Vec<_> = (0..257).map(|slot| json!(format!("0x{slot:x}"))).collect();
    assert_eq!(
        execute_rpc(
            Arc::clone(&context),
            "eth_getProof",
            vec![json!(fixture.contract), json!(too_many), json!("latest")]
        )
        .await
        .unwrap_err()
        .code(),
        -32602
    );
    assert!(
        execute_rpc(
            Arc::clone(&context),
            "eth_call",
            vec![
                json!({"from":fixture.sender,"to":fixture.contract,"gas":"0x1c9c381"}),
                json!("latest")
            ]
        )
        .await
        .is_err()
    );
    assert!(execute_rpc(Arc::clone(&context), "eth_call", vec![json!({"from":fixture.sender,"to":fixture.contract,"data":format!("0x{}", "00".repeat(131_073))}),json!("latest")]).await.is_err());
    assert_eq!(
        execute_rpc(
            Arc::clone(&context),
            "eth_getLogs",
            vec![json!({"fromBlock":"0x0","toBlock":"0x3e8"})]
        )
        .await
        .unwrap_err()
        .code(),
        -32001,
        "Unknown future history is unavailable rather than an empty successful range"
    );
    assert_eq!(
        execute_rpc(context, "eth_blockNumber", vec![])
            .await
            .unwrap(),
        json!("0x0")
    );
}
