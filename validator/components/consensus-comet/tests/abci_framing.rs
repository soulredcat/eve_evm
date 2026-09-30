use std::io::{self, Cursor};

use eve_consensus_comet::wire::{
    framing::{read_abci_request, write_abci_response},
    tendermint::abci::{Request, RequestInfo, Response, ResponseInfo, request, response},
};
use prost::Message;

#[test]
fn reads_actual_varint_delimited_request_encoding() {
    let request = Request {
        value: Some(request::Value::Info(RequestInfo {
            version: "0.40.0".into(),
            ..Default::default()
        })),
    };
    let bytes = request.encode_length_delimited_to_vec();
    assert_eq!(
        read_abci_request(&mut Cursor::new(&bytes), 1024).unwrap(),
        request
    );
}

#[test]
fn rejects_untrusted_oversize_overlong_overflow_and_truncated_frames() {
    for bytes in [vec![0x81, 0], vec![0xff; 10], vec![0x80; 10], vec![5, 1]] {
        assert!(read_abci_request(&mut Cursor::new(bytes), 1024).is_err());
    }
    assert_eq!(
        read_abci_request(&mut Cursor::new([0x80, 0x08]), 1023)
            .unwrap_err()
            .kind(),
        io::ErrorKind::InvalidData
    );
}

#[test]
fn writes_engine_compatible_length_delimited_response_with_bound() {
    let response = Response {
        value: Some(response::Value::Info(ResponseInfo {
            last_block_height: 42,
            last_block_app_hash: vec![0x42; 32],
            ..Default::default()
        })),
    };
    let mut bytes = Vec::new();
    write_abci_response(&mut bytes, &response, 1024).unwrap();
    assert_eq!(
        Response::decode_length_delimited(bytes.as_slice()).unwrap(),
        response
    );
    assert_eq!(
        write_abci_response(&mut Vec::new(), &response, 1)
            .unwrap_err()
            .kind(),
        io::ErrorKind::InvalidInput
    );
}
