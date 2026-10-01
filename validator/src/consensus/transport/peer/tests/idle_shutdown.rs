// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::spawn_authenticated_test_peer;
use crate::consensus::transport::peer::{
    EngineChannel, engine_peer_read_ready, engine_peer_shutdown_handle,
    ensure_application_engine_peer, shutdown_engine_peer_handle,
};
use std::time::{Duration, Instant};

#[test]
fn healthy_idle_peer_remains_live_and_poll_does_not_consume_or_close_stream() {
    let fixture = spawn_authenticated_test_peer(EngineChannel::Application);
    let before = Instant::now();
    assert!(!engine_peer_read_ready(&fixture.peer, Duration::from_millis(25)).unwrap());
    assert!(before.elapsed() < Duration::from_secs(1));
    ensure_application_engine_peer(&fixture.peer).unwrap();
    assert!(engine_peer_read_ready(&fixture.peer, Duration::from_secs(2)).is_err());
}

#[test]
fn sealed_close_only_capability_wakes_blocked_frame_worker_without_exposing_fd() {
    let fixture = spawn_authenticated_test_peer(EngineChannel::Application);
    let shutdown = engine_peer_shutdown_handle(&fixture.peer).unwrap();
    let mut peer = fixture;
    let worker = std::thread::spawn(move || {
        crate::consensus::transport::framing::read_engine_application_request(&mut peer.peer)
    });
    std::thread::sleep(Duration::from_millis(20));
    let before = Instant::now();
    shutdown_engine_peer_handle(&shutdown).unwrap();
    assert!(worker.join().unwrap().is_err());
    assert!(before.elapsed() < Duration::from_secs(1));
}
