// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{response::response, rpc_json};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
    time::Duration,
};

#[test]
fn real_http_content_length_and_chunked_return_only_rpc_result() {
    let body = br#"{"jsonrpc":"2.0","result":{"height":"7"}}"#;
    let mut length =
        format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len()).into_bytes();
    length.extend_from_slice(body);
    assert_eq!(response(length).unwrap()["height"], "7");
    let split = body.len() / 2;
    let mut chunked = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n".to_vec();
    for chunk in [&body[..split], &body[split..]] {
        chunked.extend_from_slice(format!("{:x};bounded=yes\r\n", chunk.len()).as_bytes());
        chunked.extend_from_slice(chunk);
        chunked.extend_from_slice(b"\r\n");
    }
    chunked.extend_from_slice(b"0\r\nX-Checked: yes\r\n\r\n");
    assert_eq!(response(chunked).unwrap()["height"], "7");
}

#[test]
fn untrusted_http_lengths_transfer_coding_truncation_and_rpc_errors_reject() {
    for bytes in [
        b"HTTP/1.1 200 OK\r\nContent-Length: 16777217\r\n\r\n".to_vec(),
        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Length: 2\r\n\r\n{}".to_vec(),
        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nTransfer-Encoding: chunked\r\n\r\n{}".to_vec(),
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n1000001\r\n".to_vec(),
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2\r\n{}XX".to_vec(),
        b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\n{}".to_vec(),
        b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\n{\"error\":1}".to_vec(),
    ] {
        assert!(response(bytes).is_err());
    }
    let remote: SocketAddr = "192.0.2.1:26657".parse().unwrap();
    assert!(rpc_json(remote, "/block").is_err());
    assert!(rpc_json("127.0.0.1:1".parse().unwrap(), "/block\r\nInjected: true").is_err());
}

#[test]
fn partial_http_progress_does_not_reset_absolute_read_deadline() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let before = std::time::Instant::now();
    let (progress, observed) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\n{")
            .unwrap();
        std::thread::sleep(Duration::from_millis(1_700));
        let advanced = stream.write_all(b"}").is_ok();
        progress.send((before.elapsed(), advanced)).unwrap();
        std::thread::sleep(Duration::from_millis(1_700));
        let _ = stream.write_all(b"\n");
    });
    let error = rpc_json(address, "/status").unwrap_err();
    let elapsed = before.elapsed();
    let io_kind = error
        .downcast_ref::<std::io::Error>()
        .map(std::io::Error::kind);
    let timed_out = matches!(
        io_kind,
        Some(std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut)
    ) || error
        .chain()
        .any(|cause| cause.to_string() == "B3_RPC_IO_TIMEOUT");
    worker.join().unwrap();
    let (second_write, advanced) = observed.recv().unwrap();
    eprintln!(
        "absolute RPC deadline: caller observed {elapsed:?}; second write {second_write:?}; deadline error {timed_out}; I/O kind {io_kind:?}"
    );
    assert!(
        advanced && second_write < Duration::from_secs(3),
        "fixture must deliver partial progress before the unchanged three-second deadline"
    );
    // Resetting the read timeout would receive valid HTTP/JSON bytes `{}` plus newline at 3.4s.
    // Its missing RPC result, EOF or another parse error cannot satisfy this deadline assertion.
    assert!(timed_out, "expected deadline failure, received {error}");
}

#[test]
fn rpc_failure_contexts_are_fixed_and_do_not_echo_untrusted_payloads() {
    let private = "untrusted-server-detail";
    for (body, expected) in [
        (format!("not-json-{private}"), "B3_RPC_JSON"),
        (
            format!(r#"{{"error":{{"data":"{private}"}},"result":{{}}}}"#),
            "B3_RPC_ERROR",
        ),
        (format!(r#"{{"unexpected":"{private}"}}"#), "B3_RPC_RESULT"),
        (r#"{"result":null}"#.to_owned(), "B3_RPC_RESULT"),
        (r#"{"result":1}"#.to_owned(), "B3_RPC_RESULT"),
    ] {
        let mut bytes =
            format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len()).into_bytes();
        bytes.extend_from_slice(body.as_bytes());
        let error = response(bytes).unwrap_err();
        assert_eq!(error.to_string(), expected);
        assert!(!format!("{error:#}").contains(private));
    }
    let error = response(b"HTTP/1.1 503 untrusted-server-detail\r\n\r\n".to_vec()).unwrap_err();
    assert_eq!(error.to_string(), "B3_RPC_READ");
    assert!(!format!("{error:#}").contains(private));
    let error = response(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\n{}".to_vec()).unwrap_err();
    assert_eq!(error.to_string(), "B3_RPC_READ");
    assert!(
        error
            .chain()
            .any(|cause| cause.to_string() == "B3_RPC_IO_DISCONNECTED")
    );
    for (kind, expected) in [
        (std::io::ErrorKind::TimedOut, "B3_RPC_IO_TIMEOUT"),
        (std::io::ErrorKind::WouldBlock, "B3_RPC_IO_TIMEOUT"),
        (
            std::io::ErrorKind::ConnectionReset,
            "B3_RPC_IO_DISCONNECTED",
        ),
        (std::io::ErrorKind::UnexpectedEof, "B3_RPC_IO_DISCONNECTED"),
        (std::io::ErrorKind::ConnectionRefused, "B3_RPC_IO_REFUSED"),
        (std::io::ErrorKind::PermissionDenied, "B3_RPC_IO_PERMISSION"),
        (std::io::ErrorKind::Other, "B3_RPC_IO_OTHER"),
    ] {
        let error =
            super::sanitize_rpc_io_error::sanitize_rpc_io_error(std::io::Error::new(kind, private));
        assert_eq!(error.kind(), kind);
        assert!(!error.to_string().contains(private));
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn slow_broadcast_response_keeps_the_three_second_absolute_deadline() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let worker = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        let mut request = Vec::new();
        while !request.ends_with(b"\r\n\r\n") {
            let mut byte = [0];
            stream.read_exact(&mut byte).unwrap();
            request.push(byte[0]);
        }
        assert!(request.starts_with(b"GET /broadcast_tx_commit?tx=0x01 HTTP/1.1\r\n"));
        std::thread::sleep(Duration::from_millis(3_400));
        let body = serde_json::to_vec(&serde_json::json!({
            "result": {
                "check_tx": {"code": 0},
                "tx_result": {"code": 0},
                "hash": hex::encode_upper([0x5a; 32]),
                "height": "7"
            }
        }))
        .unwrap();
        let header = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len());
        let _ = stream.write_all(header.as_bytes());
        let _ = stream.write_all(&body);
    });
    let error = rpc_json(address, "/broadcast_tx_commit?tx=0x01").unwrap_err();
    worker.join().unwrap();
    assert_eq!(error.to_string(), "B3_RPC_READ");
    assert!(
        error
            .chain()
            .any(|cause| cause.to_string() == "B3_RPC_IO_TIMEOUT")
    );
}
