// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::RpcReply;
use crate::support::transactions::submit_transaction_to_until::submit_transaction_to_until;
use alloy_primitives::Bytes;
use anyhow::Result;
use std::{
    io::{Read, Write},
    net::TcpListener,
    time::{Duration, Instant},
};

/// Real local TCP framing only; scripted replies cannot replace native consensus acceptance.
#[cfg(test)]
pub(in super::super) fn execute_rpc_fixture(
    replies: Vec<RpcReply>,
    budget: Duration,
) -> (Result<i64>, Vec<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    listener.set_nonblocking(true).unwrap();
    let worker = std::thread::spawn(move || {
        let deadline = Instant::now() + budget + Duration::from_secs(4);
        let mut observed = Vec::new();
        for reply in replies {
            let (mut stream, _) = loop {
                match listener.accept() {
                    Ok(connection) => break connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(Instant::now() < deadline, "scripted native request missing");
                        std::thread::sleep(Duration::from_millis(2));
                    }
                    Err(error) => panic!("fixture listener failed: {}", error.kind()),
                }
            };
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut request = Vec::new();
            while !request.ends_with(b"\r\n\r\n") {
                assert!(request.len() < 16_384);
                let mut byte = [0];
                stream.read_exact(&mut byte).unwrap();
                request.push(byte[0]);
            }
            let request = std::str::from_utf8(&request).unwrap();
            let target = request.split_ascii_whitespace().nth(1).unwrap().to_owned();
            assert_eq!(target, reply.target);
            observed.push(target);
            std::thread::sleep(reply.delay);
            let body = serde_json::to_vec(&serde_json::json!({"result": reply.result})).unwrap();
            let header = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len());
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(&body);
        }
        assert!(
            matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock),
            "unexpected native request after scripted one-shot sequence"
        );
        observed
    });
    let result = submit_transaction_to_until(
        address,
        &Bytes::from_static(&[1, 2, 3]),
        Instant::now() + budget,
    );
    (result, worker.join().unwrap())
}
