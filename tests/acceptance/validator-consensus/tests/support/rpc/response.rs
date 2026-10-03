// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::rpc_json;
use std::{
    io::{Read, Write},
    net::TcpListener,
    time::Duration,
};

#[cfg(test)]
pub(super) fn response(bytes: Vec<u8>) -> anyhow::Result<serde_json::Value> {
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
        let _ = stream.write_all(&bytes);
    });
    let result = rpc_json(address, "/block?height=1");
    worker.join().unwrap();
    result
}
