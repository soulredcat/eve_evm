// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{fixtures::fixture, leases::reserve};
use eve_storage::checkpoints::messages::CheckpointRequestKind;
use eve_sync_client::{
    NativeRpcConfig, fetch_checkpoint_response, fetch_native_frame_before,
    request_native_json_before, required_native_rpc_reservation,
};
use std::{
    net::TcpListener,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

#[test]
fn preserved_caller_deadline_refuses_http_and_native_before_connect_after_delayed_admission() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let rpc = NativeRpcConfig {
        address: listener.local_addr().unwrap(),
        timeout: Duration::from_secs(2),
        maximum_request_bytes: 16_384,
        maximum_response_bytes: 131_072,
    };
    let held = Arc::new(AtomicUsize::new(0));
    let bytes = required_native_rpc_reservation(rpc).unwrap();
    let deadline = Instant::now() + Duration::from_millis(40);
    let lease = reserve(&held, bytes).unwrap();
    // A real caller admission delay consumes the existing budget before either API is entered.
    std::thread::sleep(Duration::from_millis(80));
    assert!(
        request_native_json_before(rpc, "/status", bytes, deadline)
            .err()
            .unwrap()
            .to_string()
            .contains("TIMEOUT")
    );
    assert!(
        fetch_native_frame_before(rpc, 1, bytes, deadline)
            .err()
            .unwrap()
            .to_string()
            .contains("DEADLINE")
    );
    assert_eq!(
        listener.accept().err().unwrap().kind(),
        std::io::ErrorKind::WouldBlock
    );
    drop(lease);
    assert_eq!(held.load(Ordering::SeqCst), 0);
}
#[test]
fn snapshot_initial_caller_reservation_delay_cannot_receive_a_new_transport_timeout() {
    let fixture = fixture();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let rpc = NativeRpcConfig {
        address: listener.local_addr().unwrap(),
        timeout: Duration::from_millis(40),
        maximum_request_bytes: 16_384,
        maximum_response_bytes: 131_072,
    };
    let held = Arc::new(AtomicUsize::new(0));
    let mut calls = 0;
    let result = fetch_checkpoint_response(
        rpc,
        &fixture.request(CheckpointRequestKind::Execution),
        &fixture.limits,
        &mut |bytes| {
            calls += 1;
            std::thread::sleep(Duration::from_millis(80));
            reserve(&held, bytes)
        },
    );
    assert!(result.err().unwrap().to_string().contains("DEADLINE"));
    assert_eq!(calls, 1);
    assert_eq!(
        listener.accept().err().unwrap().kind(),
        std::io::ErrorKind::WouldBlock
    );
    assert_eq!(held.load(Ordering::SeqCst), 0);
}
