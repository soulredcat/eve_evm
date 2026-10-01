// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        EngineChannel, authenticate_engine_peer, engine_peer_read_ready, validate_engine_peer_child,
    },
    spawn_test_connection,
};
use crate::consensus::transport::framing::{
    read_engine_application_request, read_engine_signer_request, write_engine_application_response,
    write_engine_signer_response,
};
use eve_consensus_comet::wire::tendermint::{abci, privval};
use std::{
    io::{ErrorKind, Write},
    time::{Duration, Instant},
};

#[test]
fn actual_authenticated_application_and_signer_streams_roundtrip_native_types() {
    for channel in [EngineChannel::Application, EngineChannel::Signer] {
        let mut fixture = spawn_test_connection(if channel == EngineChannel::Application {
            "application"
        } else {
            "signer"
        });
        let mut control = fixture.stream.as_ref().unwrap().try_clone().unwrap();
        let mut peer = authenticate_engine_peer(
            fixture.stream.take().unwrap(),
            fixture.child.as_mut().unwrap(),
            &fixture.image,
            channel,
            Duration::from_secs(5),
            Duration::from_secs(5),
        )
        .unwrap();
        control.write_all(b"!").unwrap();
        validate_engine_peer_child(&peer, fixture.child.as_mut().unwrap()).unwrap();
        assert!(engine_peer_read_ready(&peer, Duration::from_secs(1)).unwrap());
        assert!(engine_peer_read_ready(&peer, Duration::ZERO).unwrap());
        if channel == EngineChannel::Application {
            let request = read_engine_application_request(&mut peer).unwrap();
            validate_engine_peer_child(&peer, fixture.child.as_mut().unwrap()).unwrap();
            assert!(matches!(request.value, Some(abci::request::Value::Echo(_))));
            let response = abci::Response {
                value: Some(abci::response::Value::Echo(abci::ResponseEcho {
                    message: "reply".into(),
                })),
            };
            write_engine_application_response(&mut peer, &response).unwrap();
        } else {
            let request = read_engine_signer_request(&mut peer).unwrap();
            validate_engine_peer_child(&peer, fixture.child.as_mut().unwrap()).unwrap();
            assert!(matches!(
                request.sum,
                Some(privval::message::Sum::PingRequest(_))
            ));
            let response = privval::Message {
                sum: Some(privval::message::Sum::PingResponse(
                    privval::PingResponse {},
                )),
            };
            write_engine_signer_response(&mut peer, &response).unwrap();
        }
        assert!(fixture.child.as_mut().unwrap().wait().unwrap().success());
    }
}

#[test]
fn authenticated_stream_rejects_bad_lengths_truncation_and_sanitizes_protobuf_faults() {
    for mode in [
        "overlong",
        "oversized-application",
        "truncated-prefix",
        "truncated-body",
        "malformed",
    ] {
        let mut fixture = spawn_test_connection(mode);
        let mut control = fixture.stream.as_ref().unwrap().try_clone().unwrap();
        let mut peer = authenticate_engine_peer(
            fixture.stream.take().unwrap(),
            fixture.child.as_mut().unwrap(),
            &fixture.image,
            EngineChannel::Application,
            Duration::from_secs(5),
            Duration::from_secs(5),
        )
        .unwrap();
        control.write_all(b"!").unwrap();
        let error = read_engine_application_request(&mut peer).unwrap_err();
        assert!(
            matches!(
                error.kind(),
                ErrorKind::InvalidData | ErrorKind::UnexpectedEof
            ),
            "mode {mode}"
        );
        if mode == "malformed" {
            assert_eq!(error.to_string(), "invalid native protobuf message");
        }
        assert!(
            read_engine_application_request(&mut peer).is_err(),
            "fault closes the same stream"
        );
    }
}

#[test]
fn signer_frame_cap_and_channel_boundary_are_enforced_on_real_stream() {
    let mut fixture = spawn_test_connection("oversized-signer");
    let mut control = fixture.stream.as_ref().unwrap().try_clone().unwrap();
    let mut peer = authenticate_engine_peer(
        fixture.stream.take().unwrap(),
        fixture.child.as_mut().unwrap(),
        &fixture.image,
        EngineChannel::Signer,
        Duration::from_secs(5),
        Duration::from_secs(5),
    )
    .unwrap();
    assert_eq!(
        read_engine_application_request(&mut peer)
            .unwrap_err()
            .kind(),
        ErrorKind::PermissionDenied
    );
    control.write_all(b"!").unwrap();
    assert_eq!(
        read_engine_signer_request(&mut peer).unwrap_err().kind(),
        ErrorKind::InvalidData
    );
}

#[test]
fn slow_progress_does_not_reset_the_absolute_native_frame_deadline() {
    let mut fixture = spawn_test_connection("slow-progress");
    let mut control = fixture.stream.as_ref().unwrap().try_clone().unwrap();
    let mut peer = authenticate_engine_peer(
        fixture.stream.take().unwrap(),
        fixture.child.as_mut().unwrap(),
        &fixture.image,
        EngineChannel::Application,
        Duration::from_millis(150),
        Duration::from_secs(5),
    )
    .unwrap();
    control.write_all(b"!").unwrap();
    let started = Instant::now();
    let error = read_engine_application_request(&mut peer).unwrap_err();
    assert!(matches!(
        error.kind(),
        ErrorKind::TimedOut | ErrorKind::WouldBlock
    ));
    assert!(started.elapsed() < Duration::from_secs(1));
}

#[test]
fn oversized_responses_fail_before_native_message_serialization_or_write() {
    let mut fixture = super::spawn_authenticated_test_peer(EngineChannel::Application);
    let response = abci::Response {
        value: Some(abci::response::Value::Echo(abci::ResponseEcho {
            message: "x".repeat(4 * 1_048_576 + 65_536),
        })),
    };
    assert_eq!(
        write_engine_application_response(&mut fixture.peer, &response)
            .unwrap_err()
            .kind(),
        ErrorKind::InvalidInput
    );
}
