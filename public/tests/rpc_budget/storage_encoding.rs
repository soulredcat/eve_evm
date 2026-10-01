// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::execute_rpc::execute_rpc;
use alloy_primitives::Bytes;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn ta01_storage_positions_accept_canonical_zero_padded_words_without_weakening_block_quantities()
 {
    let fixture = fixture(Bytes::new());
    let context = fixture.context;
    for slot in [
        "0x0".to_string(),
        format!("0x{:064x}", 0),
        format!("0x{:064x}", 999),
    ] {
        assert_eq!(
            execute_rpc(
                Arc::clone(&context),
                "eth_getStorageAt",
                vec![json!(fixture.contract), json!(slot), json!("latest")]
            )
            .await
            .unwrap(),
            json!(format!("0x{}", "00".repeat(32)))
        );
        let proof = execute_rpc(
            Arc::clone(&context),
            "eth_getProof",
            vec![json!(fixture.contract), json!([slot]), json!("latest")],
        )
        .await
        .unwrap();
        assert_eq!(proof["storageProof"][0]["value"], "0x0");
    }
    assert_eq!(
        execute_rpc(
            context,
            "eth_getBalance",
            vec![json!(fixture.sender), json!("0x00")]
        )
        .await
        .unwrap_err()
        .code(),
        -32602
    );
}
