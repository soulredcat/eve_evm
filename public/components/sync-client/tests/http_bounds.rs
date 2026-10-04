// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_sync_client::{NativeRpcConfig, request_native_json, required_native_rpc_reservation};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
    thread,
    time::Duration,
};

fn config(address: SocketAddr) -> NativeRpcConfig {
    NativeRpcConfig {
        address,
        timeout: Duration::from_secs(1),
        maximum_request_bytes: 512,
        maximum_response_bytes: 1024,
    }
}
fn server(response: Vec<u8>, delay: Duration) -> (SocketAddr, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let worker = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let mut request = Vec::new();
        while !request.ends_with(b"\r\n\r\n") && request.len() < 512 {
            let mut byte = [0];
            if stream.read(&mut byte).unwrap_or(0) == 0 {
                return;
            }
            request.push(byte[0]);
        }
        thread::sleep(delay);
        let _ = stream.write_all(&response);
    });
    (address, worker)
}
fn fetch(response: &[u8]) -> anyhow::Result<serde_json::Value> {
    let (address, worker) = server(response.to_vec(), Duration::ZERO);
    let config = config(address);
    let value = request_native_json(
        config,
        "/status",
        required_native_rpc_reservation(config).unwrap(),
    );
    worker.join().unwrap();
    value
}

#[test]
fn real_content_length_and_chunked_transports_return_the_same_untrusted_result() {
    let body = br#"{"result":{"height":"7"}}"#;
    let framed = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len());
    let plain = [framed.as_bytes(), body].concat();
    let chunked = format!(
        "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n{:x}\r\n{}\r\n0\r\n\r\n",
        body.len(),
        std::str::from_utf8(body).unwrap()
    );
    assert_eq!(fetch(&plain).unwrap(), fetch(chunked.as_bytes()).unwrap());
}
#[test]
fn oversized_length_conflicting_framing_compression_and_remote_errors_refuse() {
    for response in [
        b"HTTP/1.1 200 OK\r\nContent-Length: 1025\r\n\r\n".as_slice(),
        b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nContent-Length: 0\r\n\r\n",
        b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nTransfer-Encoding: chunked\r\n\r\n",
        b"HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: 0\r\n\r\n",
        b"HTTP/1.1 503 unavailable\r\nContent-Length: 0\r\n\r\n",
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n401\r\n",
        b"HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n{\"error\":true}",
    ] {
        assert!(fetch(response).is_err());
    }
}
#[test]
fn unsafe_path_endpoint_missing_reservation_and_overflow_refuse_before_connect() {
    let config = config("127.0.0.1:1".parse().unwrap());
    let required = required_native_rpc_reservation(config).unwrap();
    assert!(
        request_native_json(config, "/status", required - 1)
            .unwrap_err()
            .to_string()
            .contains("RESERVATION")
    );
    assert!(
        request_native_json(config, "/status\r\nInjected", required)
            .unwrap_err()
            .to_string()
            .contains("REQUEST")
    );
    let mut remote = config;
    remote.address = "192.0.2.1:80".parse().unwrap();
    assert!(
        request_native_json(remote, "/status", required)
            .unwrap_err()
            .to_string()
            .contains("ENDPOINT")
    );
    let mut overflow = config;
    overflow.maximum_response_bytes = usize::MAX;
    assert!(required_native_rpc_reservation(overflow).is_err());
}
#[test]
fn slow_actual_peer_stops_at_the_declared_total_deadline() {
    let (address, worker) = server(
        b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n".to_vec(),
        Duration::from_millis(80),
    );
    let mut config = config(address);
    config.timeout = Duration::from_millis(20);
    assert!(
        request_native_json(
            config,
            "/status",
            required_native_rpc_reservation(config).unwrap()
        )
        .is_err()
    );
    worker.join().unwrap();
}
