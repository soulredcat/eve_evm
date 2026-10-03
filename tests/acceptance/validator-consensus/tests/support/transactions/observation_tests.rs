// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    submit_transaction_to_until::submit_transaction_to_until,
    test_support::{
        RpcReply, build_admission_response, build_native_block_response, execute_rpc_fixture,
    },
};
use alloy_primitives::Bytes;
use serde_json::{Value, json};
use std::{
    net::TcpListener,
    time::{Duration, Instant},
};

#[cfg(test)]
fn reply(target: &str, result: Value) -> RpcReply {
    RpcReply {
        target: target.into(),
        result,
        delay: Duration::ZERO,
    }
}

#[cfg(test)]
fn ready_replies(result_delay: Duration) -> Vec<RpcReply> {
    let mut replies = vec![
        reply(
            "/status",
            json!({"sync_info": {"latest_block_height": "2"}}),
        ),
        reply(
            "/broadcast_tx_sync?tx=0x010203",
            build_admission_response(&[1, 2, 3]),
        ),
        reply(
            "/status",
            json!({"sync_info": {"latest_block_height": "3"}}),
        ),
        reply(
            "/abci_info",
            json!({"response": {"last_block_height": "3"}}),
        ),
        reply(
            "/block?height=3",
            build_native_block_response(3, &[vec![9], vec![1, 2, 3]]),
        ),
        reply(
            "/block_results?height=3",
            json!({"height": "3", "txs_results": [{"code": 1}, {"code": 0}]}),
        ),
    ];
    replies.last_mut().unwrap().delay = result_delay;
    replies
}

#[test]
fn one_shot_admission_waits_for_application_and_exact_native_execution_result() {
    let mut replies = ready_replies(Duration::ZERO);
    replies[3].result = json!({"response": {"last_block_height": "2"}});
    replies.splice(
        4..4,
        [
            reply(
                "/status",
                json!({"sync_info": {"latest_block_height": "4"}}),
            ),
            reply(
                "/abci_info",
                json!({"response": {"last_block_height": "3"}}),
            ),
        ],
    );
    let (result, observed) = execute_rpc_fixture(replies, Duration::from_secs(5));
    assert_eq!(result.unwrap(), 3);
    assert_eq!(observed.len(), 8);
    assert_eq!(
        observed
            .iter()
            .filter(|target| target.starts_with("/broadcast_tx_sync"))
            .count(),
        1
    );
    assert!(
        !observed
            .iter()
            .any(|target| target.starts_with("/tx?") || target.starts_with("/broadcast_tx_commit"))
    );
    assert_eq!(observed.last().unwrap(), "/block_results?height=3");
    for (field, value, expected) in [
        ("code", json!(1), "B3_SUBMIT_CHECK_TX_REJECTED"),
        ("code", json!("0"), "B3_SUBMIT_CHECK_TX_CODE"),
        (
            "hash",
            json!(hex::encode([0; 32])),
            "B3_SUBMIT_HASH_MISMATCH",
        ),
    ] {
        let mut replies = ready_replies(Duration::ZERO);
        replies.truncate(2);
        replies[1].result[field] = value;
        let (result, observed) = execute_rpc_fixture(replies, Duration::from_secs(5));
        assert_eq!(result.unwrap_err().to_string(), expected);
        assert_eq!(observed.len(), 2);
    }
}

#[test]
fn ambiguous_admission_timeout_is_not_rebroadcast() {
    let mut replies = ready_replies(Duration::ZERO);
    replies.truncate(2);
    replies[1].delay = Duration::from_millis(3_400);
    let (result, observed) = execute_rpc_fixture(replies, Duration::from_secs(5));
    let error = result.unwrap_err();
    assert_eq!(error.to_string(), "B3_RPC_READ");
    assert!(
        error
            .chain()
            .any(|cause| cause.to_string() == "B3_RPC_IO_TIMEOUT")
    );
    assert_eq!(observed.len(), 2);
    assert_eq!(
        observed
            .iter()
            .filter(|target| target.starts_with("/broadcast_tx_sync"))
            .count(),
        1
    );
}

#[test]
fn shared_deadline_rejects_late_native_execution_result() {
    let replies = ready_replies(Duration::from_millis(750));
    let (result, observed) = execute_rpc_fixture(replies, Duration::from_millis(500));
    assert_eq!(
        result.unwrap_err().to_string(),
        "B3_SUBMIT_PROGRESS_DEADLINE"
    );
    assert_eq!(observed.last().unwrap(), "/block_results?height=3");
    assert_eq!(
        observed
            .iter()
            .filter(|target| target.starts_with("/broadcast_tx_sync"))
            .count(),
        1
    );
}

#[test]
fn expired_progress_budget_makes_no_native_request() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let result = submit_transaction_to_until(
        listener.local_addr().unwrap(),
        &Bytes::from_static(&[1, 2, 3]),
        Instant::now() - Duration::from_millis(1),
    );
    assert_eq!(
        result.unwrap_err().to_string(),
        "B3_SUBMIT_PROGRESS_DEADLINE"
    );
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}
