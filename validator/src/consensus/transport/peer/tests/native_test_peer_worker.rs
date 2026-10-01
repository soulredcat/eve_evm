// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::wire::{
    framing::{read_protobuf_message, write_protobuf_message},
    tendermint::{abci, privval},
};
use prost::Message;
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    time::Duration,
};

#[test]
fn native_test_peer_worker() {
    let Some(path) = std::env::var_os("EVE_NATIVE_TEST_PEER_PATH") else {
        let mut fixture =
            super::spawn_authenticated_test_peer(super::super::EngineChannel::Application);
        super::super::ensure_application_engine_peer(&fixture.peer).unwrap();
        super::super::shutdown_engine_peer(&fixture.peer).unwrap();
        assert!(
            fixture.child.wait().unwrap().success(),
            "actual test-worker shutdown must be orderly"
        );
        return;
    };
    let mut stream = UnixStream::connect(path).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(15)))
        .unwrap();
    if stream.read_exact(&mut [0]).is_err() {
        std::process::exit(0);
    }
    let mode = std::env::var("EVE_NATIVE_TEST_PEER_MODE").unwrap();
    match mode.as_str() {
        "application" => {
            let request = abci::Request {
                value: Some(abci::request::Value::Echo(abci::RequestEcho {
                    message: "real stream".into(),
                })),
            };
            write_protobuf_message(&mut stream, &request, 1024).unwrap();
            let response: abci::Response = read_protobuf_message(&mut stream, 1024).unwrap();
            assert!(matches!(
                response.value,
                Some(abci::response::Value::Echo(_))
            ));
        }
        "signer" => {
            let request = privval::Message {
                sum: Some(privval::message::Sum::PingRequest(privval::PingRequest {})),
            };
            write_protobuf_message(&mut stream, &request, 1024).unwrap();
            let response: privval::Message = read_protobuf_message(&mut stream, 1024).unwrap();
            assert!(matches!(
                response.sum,
                Some(privval::message::Sum::PingResponse(_))
            ));
        }
        "overlong" => {
            stream.write_all(&[0x80, 0]).unwrap();
        }
        "oversized-signer" => {
            stream.write_all(&[0x81, 0x80, 0x04]).unwrap();
        }
        "oversized-application" => {
            stream.write_all(&[0x81, 0x80, 0x84, 0x02]).unwrap();
        }
        "malformed" => {
            stream.write_all(&[1, 0xff]).unwrap();
        }
        "truncated-prefix" => {
            stream.write_all(&[0x80]).unwrap();
            stream.shutdown(std::net::Shutdown::Write).unwrap();
        }
        "truncated-body" => {
            stream.write_all(&[4, 8, 1]).unwrap();
            stream.shutdown(std::net::Shutdown::Write).unwrap();
        }
        "slow-progress" => {
            let request = abci::Request {
                value: Some(abci::request::Value::Echo(abci::RequestEcho {
                    message: "slow".into(),
                })),
            };
            for byte in request.encode_length_delimited_to_vec() {
                if stream.write_all(&[byte]).is_err() {
                    break;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        }
        "hold" => {}
        _ => panic!("unknown test peer mode"),
    }
    if !matches!(mode.as_str(), "application" | "signer") {
        let _ = stream.read(&mut [0]);
    }
    std::process::exit(0);
}
