// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use base64::{Engine, engine::general_purpose::STANDARD};
use eve_state::{StateDeltaChunk, decode_state_delta_request, encode_state_delta_chunk};
use eve_sync_client::NativeRpcConfig;
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

pub(super) fn server(chunks: Vec<StateDeltaChunk>) -> (NativeRpcConfig, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let worker = thread::spawn(move || {
        for chunk in chunks {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(1)))
                .unwrap();
            let mut request = Vec::new();
            while !request.ends_with(b"\r\n\r\n") && request.len() < 16_384 {
                let mut byte = [0];
                if stream.read(&mut byte).unwrap_or(0) == 0 {
                    return;
                }
                request.push(byte[0]);
            }
            let path = std::str::from_utf8(&request)
                .unwrap()
                .split_ascii_whitespace()
                .nth(1)
                .unwrap();
            let encoded = path
                .split("&data=0x")
                .nth(1)
                .unwrap()
                .split('&')
                .next()
                .unwrap();
            let decoded = decode_state_delta_request(&hex::decode(encoded).unwrap()).unwrap();
            assert_eq!(decoded.target_height, 1);
            let encoded = encode_state_delta_chunk(&chunk).unwrap();
            let body = serde_json::to_vec(&serde_json::json!({"result":{"response":{"code":0,"value":STANDARD.encode(encoded)}}})).unwrap();
            let header = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len());
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(&body);
        }
    });
    (
        NativeRpcConfig {
            address,
            timeout: Duration::from_secs(1),
            maximum_request_bytes: 16_384,
            maximum_response_bytes: 16_384,
        },
        worker,
    )
}
