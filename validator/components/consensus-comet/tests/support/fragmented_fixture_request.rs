// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use eve_consensus_comet::wire::tendermint::abci::{
    Request, RequestInfo, Response, request, response,
};
use prost::Message;

use super::{FixtureState, start_fixture_server};

#[test]
fn blocking_fixture_reader_preserves_fragmented_varint_and_payload() {
    let state = Arc::new(Mutex::new(FixtureState::default()));
    let server = start_fixture_server(
        "127.0.0.1:0".parse().unwrap(),
        state,
        std::path::PathBuf::new(),
    )
    .unwrap();
    let mut client = TcpStream::connect(server.address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    let request = Request {
        value: Some(request::Value::Info(RequestInfo {
            version: "fragment".repeat(50),
            ..Default::default()
        })),
    };
    let frame = request.encode_length_delimited_to_vec();
    assert_ne!(frame[0] & 0x80, 0);
    client.write_all(&frame[..1]).unwrap();
    thread::sleep(Duration::from_millis(300));
    client.write_all(&frame[1..6]).unwrap();
    thread::sleep(Duration::from_millis(300));
    client.write_all(&frame[6..]).unwrap();
    let mut length = [0_u8; 1];
    client.read_exact(&mut length).unwrap();
    assert_eq!(length[0] & 0x80, 0);
    let mut response = vec![0; usize::from(length[0])];
    client.read_exact(&mut response).unwrap();
    assert!(matches!(
        Response::decode(response.as_slice()).unwrap().value,
        Some(response::Value::Info(_))
    ));
}
