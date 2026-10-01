// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::wire::{
    framing::{read_abci_request, read_protobuf_message, write_protobuf_message},
    tendermint::{abci, privval},
};
use std::io::{Cursor, ErrorKind};

#[test]
fn native_generic_framing_preserves_abci_and_privval_typed_messages() {
    let request = abci::Request {
        value: Some(abci::request::Value::Echo(abci::RequestEcho {
            message: "native".into(),
        })),
    };
    let mut bytes = Vec::new();
    write_protobuf_message(&mut bytes, &request, 1024).unwrap();
    assert_eq!(
        read_abci_request(&mut Cursor::new(&bytes), 1024).unwrap(),
        request
    );
    let signer = privval::Message {
        sum: Some(privval::message::Sum::PingRequest(privval::PingRequest {})),
    };
    bytes.clear();
    write_protobuf_message(&mut bytes, &signer, 1024).unwrap();
    let decoded: privval::Message = read_protobuf_message(&mut Cursor::new(&bytes), 1024).unwrap();
    assert_eq!(decoded, signer);
}

#[test]
fn native_generic_reader_rejects_u64_max_before_allocation_and_consumes_only_prefix() {
    let mut bytes = vec![0xff; 9];
    bytes.push(1);
    let mut reader = Cursor::new(bytes);
    let error = read_protobuf_message::<_, privval::Message>(&mut reader, 65_536).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::InvalidData);
    assert_eq!(reader.position(), 10);
    let mut reader = Cursor::new(vec![0x81, 0x80, 0x04, 0x12]);
    assert!(read_protobuf_message::<_, privval::Message>(&mut reader, 65_536).is_err());
    assert_eq!(reader.position(), 3, "oversized body is not read");
}

#[test]
fn native_generic_reader_preserves_length_fault_kinds_and_safe_decode_category() {
    for (bytes, kind) in [
        (vec![0x80, 0], ErrorKind::InvalidData),
        (vec![0xff; 10], ErrorKind::InvalidData),
        (vec![0x80], ErrorKind::UnexpectedEof),
        (vec![2, 0xff], ErrorKind::UnexpectedEof),
        (vec![1, 0xff], ErrorKind::InvalidData),
    ] {
        let error = read_protobuf_message::<_, privval::Message>(&mut Cursor::new(bytes), 1024)
            .unwrap_err();
        assert_eq!(error.kind(), kind);
    }
    assert_eq!(
        read_protobuf_message::<_, privval::Message>(&mut Cursor::new(vec![1, 0xff]), 1024)
            .unwrap_err()
            .to_string(),
        "invalid native protobuf message"
    );
}

#[test]
fn native_generic_writer_rejects_oversize_before_writing_prefix() {
    let request = abci::Request {
        value: Some(abci::request::Value::Echo(abci::RequestEcho {
            message: "over budget".into(),
        })),
    };
    let mut bytes = Vec::new();
    assert_eq!(
        write_protobuf_message(&mut bytes, &request, 1)
            .unwrap_err()
            .kind(),
        ErrorKind::InvalidInput
    );
    assert!(bytes.is_empty());
}
