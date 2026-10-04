// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    fixtures::fixture,
    native_fixture,
    server::{json, server},
};
use eve_finality_verifier::{CheckpointLimits, MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES};
use eve_storage::checkpoints::messages::CheckpointRequestKind;
use eve_sync_client::{
    CheckpointWitnessHeights, NativeRpcConfig, fetch_checkpoint_response_before,
    fetch_checkpoint_witness_before, fetch_native_frame_before, request_native_json_before,
    required_native_rpc_reservation,
};
use std::{
    net::TcpListener,
    time::{Duration, Instant},
};

#[test]
fn expired_outer_checkpoint_deadline_refuses_before_any_caller_reservation_or_connect() {
    let fixture = fixture();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let rpc = NativeRpcConfig {
        address: listener.local_addr().unwrap(),
        timeout: Duration::from_secs(2),
        maximum_request_bytes: 16_384,
        maximum_response_bytes: 131_072,
    };
    let expired = Instant::now()
        .checked_sub(Duration::from_millis(1))
        .unwrap();
    let mut calls = 0;
    let response = fetch_checkpoint_response_before(
        rpc,
        &fixture.request(CheckpointRequestKind::Execution),
        &fixture.limits,
        &mut |_| {
            calls += 1;
            Ok(())
        },
        expired,
    );
    assert!(response.err().unwrap().to_string().contains("DEADLINE"));
    let witness = fetch_checkpoint_witness_before(
        rpc,
        &fixture.chain.commits[0].target,
        CheckpointWitnessHeights {
            checkpoint: 1,
            witness: 2,
        },
        &fixture.limits.logical,
        CheckpointLimits {
            maximum_height_gap: 64,
            maximum_witness_bytes: MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES,
        },
        &mut |_| {
            calls += 1;
            Ok(())
        },
        expired,
    );
    assert!(witness.err().unwrap().to_string().contains("DEADLINE"));
    assert_eq!(calls, 0);
    assert_eq!(
        listener.accept().err().unwrap().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
#[test]
fn future_outer_checkpoint_deadline_cannot_extend_configured_callback_budget() {
    let fixture = fixture();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let rpc = NativeRpcConfig {
        address: listener.local_addr().unwrap(),
        timeout: Duration::from_millis(40),
        maximum_request_bytes: 16_384,
        maximum_response_bytes: 131_072,
    };
    let future = Instant::now() + Duration::from_secs(3_600);
    let response = fetch_checkpoint_response_before(
        rpc,
        &fixture.request(CheckpointRequestKind::Execution),
        &fixture.limits,
        &mut |_| {
            std::thread::sleep(Duration::from_millis(80));
            Ok(())
        },
        future,
    );
    assert!(response.err().unwrap().to_string().contains("DEADLINE"));
    let witness = fetch_checkpoint_witness_before(
        rpc,
        &fixture.chain.commits[0].target,
        CheckpointWitnessHeights {
            checkpoint: 1,
            witness: 2,
        },
        &fixture.limits.logical,
        CheckpointLimits {
            maximum_height_gap: 64,
            maximum_witness_bytes: MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES,
        },
        &mut |_| {
            std::thread::sleep(Duration::from_millis(80));
            Ok(())
        },
        future,
    );
    assert!(witness.err().unwrap().to_string().contains("DEADLINE"));
    assert_eq!(
        listener.accept().err().unwrap().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
#[test]
fn future_outer_deadline_cannot_extend_configured_http_or_native_io_budget() {
    let fixture = fixture();
    let (mut rpc, peer) = server(vec![
        json(
            native_fixture::block(&fixture.chain, 1),
            Duration::from_millis(300),
            "/block?",
        ),
        json(
            native_fixture::commit(&fixture.chain, 1),
            Duration::from_millis(300),
            "/commit?",
        ),
    ]);
    rpc.timeout = Duration::from_millis(500);
    let future = Instant::now() + Duration::from_secs(3_600);
    let native = fetch_native_frame_before(
        rpc,
        1,
        required_native_rpc_reservation(rpc).unwrap(),
        future,
    );
    peer.join().unwrap();
    assert!(native.is_err());
    let (mut rpc, peer) = server(vec![json(
        serde_json::json!({"result":{"height":"1"}}),
        Duration::from_millis(80),
        "/status",
    )]);
    rpc.timeout = Duration::from_millis(40);
    let http = request_native_json_before(
        rpc,
        "/status",
        required_native_rpc_reservation(rpc).unwrap(),
        future,
    );
    peer.join().unwrap();
    assert!(http.is_err());
}
