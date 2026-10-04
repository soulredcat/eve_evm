// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use base64::{Engine, engine::general_purpose::STANDARD};
use eve_storage::checkpoints::messages::decode_checkpoint_request;
use eve_sync_client::NativeRpcConfig;
use serde_json::Value;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

pub(super) struct Reply {
    pub body: Vec<u8>,
    pub delay: Duration,
    pub expected_path: &'static str,
}
pub(super) fn query(bytes: &[u8]) -> Reply {
    let value =
        serde_json::json!({"result":{"response":{"code":0,"value":STANDARD.encode(bytes)}}});
    json(value, Duration::ZERO, "/abci_query?")
}
pub(super) fn json(value: Value, delay: Duration, expected_path: &'static str) -> Reply {
    Reply {
        body: serde_json::to_vec(&value).unwrap(),
        delay,
        expected_path,
    }
}
/// Test-only socket framing. Production decoding always uses the maintained client.
pub(super) fn server(replies: Vec<Reply>) -> (NativeRpcConfig, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let worker = thread::spawn(move || {
        for reply in replies {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            assert!(line.len() <= 16_384);
            let path = line.split_ascii_whitespace().nth(1).unwrap();
            assert!(path.starts_with(reply.expected_path));
            if path.starts_with("/abci_query?") {
                let data = path
                    .split("&data=0x")
                    .nth(1)
                    .unwrap()
                    .split('&')
                    .next()
                    .unwrap();
                let request = decode_checkpoint_request(&hex::decode(data).unwrap()).unwrap();
                assert_eq!(request.genesis.height, 0);
                assert!(request.height > 0);
            }
            loop {
                line.clear();
                if reader.read_line(&mut line).unwrap_or(0) == 0 {
                    return;
                }
                assert!(line.len() <= 16_384);
                if line == "\r\n" {
                    break;
                }
            }
            thread::sleep(reply.delay);
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                reply.body.len()
            );
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(&reply.body);
        }
    });
    (
        NativeRpcConfig {
            address,
            timeout: Duration::from_secs(2),
            maximum_request_bytes: 16_384,
            maximum_response_bytes: 131_072,
        },
        worker,
    )
}
