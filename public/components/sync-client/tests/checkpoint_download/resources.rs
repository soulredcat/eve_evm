// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    fixtures::fixture,
    leases::reserve,
    server::{json, query, server},
};
use eve_storage::checkpoints::messages::CheckpointRequestKind;
use eve_sync_client::{NativeRpcConfig, fetch_checkpoint_response};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

#[test]
fn real_query_refusals_report_fixed_categories_without_echoing_peer_text() {
    let fixture = fixture();
    for (code, expected) in [
        (1, "WRONG_NETWORK"),
        (2, "UNSUPPORTED"),
        (3, "GAP"),
        (4, "RESOURCE"),
        (5, "NOT_READY"),
        (6, "MALFORMED"),
        (99, "REFUSAL"),
    ] {
        let response = serde_json::json!({"result":{"response":{"code":code,"codespace":"private-peer-diagnostic",
            "log":"private-peer-diagnostic","value":""}}});
        let (rpc, peer) = server(vec![json(response, Duration::ZERO, "/abci_query?")]);
        let result = fetch_checkpoint_response(
            rpc,
            &fixture.request(CheckpointRequestKind::Execution),
            &fixture.limits,
            &mut |_| Ok(()),
        );
        peer.join().unwrap();
        let error = result.err().unwrap().to_string();
        assert!(error.contains(expected));
        assert!(!error.contains("private-peer-diagnostic"));
    }
}
#[test]
fn actual_reservation_denial_precedes_connect_or_owned_decode_and_releases_previous_charge() {
    let fixture = fixture();
    let rpc = NativeRpcConfig {
        address: "127.0.0.1:1".parse().unwrap(),
        timeout: Duration::from_secs(1),
        maximum_request_bytes: 16_384,
        maximum_response_bytes: 131_072,
    };
    let mut calls = 0;
    let first = fetch_checkpoint_response(
        rpc,
        &fixture.request(CheckpointRequestKind::Execution),
        &fixture.limits,
        &mut |_| -> anyhow::Result<()> {
            calls += 1;
            anyhow::bail!("CALLER_CAPACITY");
        },
    );
    assert_eq!(calls, 1);
    assert!(first.err().unwrap().to_string().contains("CALLER_CAPACITY"));
    let (rpc, peer) = server(vec![query(&fixture.encoded(&fixture.execution_response()))]);
    let held = Arc::new(AtomicUsize::new(0));
    let mut calls = 0;
    let second = fetch_checkpoint_response(
        rpc,
        &fixture.request(CheckpointRequestKind::Execution),
        &fixture.limits,
        &mut |bytes| {
            calls += 1;
            if calls == 2 {
                anyhow::bail!("CALLER_CAPACITY");
            }
            reserve(&held, bytes)
        },
    );
    peer.join().unwrap();
    assert_eq!(calls, 2);
    assert!(
        second
            .err()
            .unwrap()
            .to_string()
            .contains("CALLER_CAPACITY")
    );
    assert_eq!(held.load(Ordering::SeqCst), 0);
}
#[test]
fn slow_peer_and_late_decode_admission_share_one_absolute_deadline() {
    let fixture = fixture();
    let encoded = fixture.encoded(&fixture.execution_response());
    let (mut rpc, peer) = server(vec![super::server::Reply {
        body: super::server::query(&encoded).body,
        delay: Duration::from_millis(600),
        expected_path: "/abci_query?",
    }]);
    rpc.timeout = Duration::from_millis(500);
    assert!(
        fetch_checkpoint_response(
            rpc,
            &fixture.request(CheckpointRequestKind::Execution),
            &fixture.limits,
            &mut |_| Ok(())
        )
        .is_err()
    );
    peer.join().unwrap();
    let (mut rpc, peer) = server(vec![query(&encoded)]);
    rpc.timeout = Duration::from_millis(500);
    let held = Arc::new(AtomicUsize::new(0));
    let mut calls = 0;
    let delayed = fetch_checkpoint_response(
        rpc,
        &fixture.request(CheckpointRequestKind::Execution),
        &fixture.limits,
        &mut |bytes| {
            calls += 1;
            if calls == 2 {
                std::thread::sleep(Duration::from_millis(600));
            }
            reserve(&held, bytes)
        },
    );
    peer.join().unwrap();
    assert!(delayed.err().unwrap().to_string().contains("DEADLINE"));
    assert_eq!(calls, 2);
    assert_eq!(held.load(Ordering::SeqCst), 0);
}
