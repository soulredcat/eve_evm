// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::fixture;
use eve_storage::checkpoints::messages::{
    CheckpointMessageError, CheckpointRequest, CheckpointRequestKind, decode_checkpoint_request,
    encode_checkpoint_request,
};
#[test]
fn all_request_kinds_roundtrip_canonical_genesis_and_exact_fixed_fields() {
    let (chain, _, _, _, id) = fixture();
    for kind in [
        CheckpointRequestKind::Manifest { chunk_bytes: 64 },
        CheckpointRequestKind::Chunk {
            chunk_bytes: 64,
            manifest_id: id,
            index: 1,
        },
        CheckpointRequestKind::Execution,
    ] {
        let request = CheckpointRequest {
            genesis: chain.commits[0].target.clone(),
            height: 1,
            kind,
        };
        let bytes = encode_checkpoint_request(&request).unwrap();
        assert_eq!(decode_checkpoint_request(&bytes).unwrap(), request);
        assert_eq!(
            encode_checkpoint_request(&decode_checkpoint_request(&bytes).unwrap()).unwrap(),
            bytes
        );
    }
}
#[test]
fn unsupported_version_and_compression_unknown_kind_and_trailing_bytes_are_rejected() {
    let (chain, _, _, _, _) = fixture();
    let request = CheckpointRequest {
        genesis: chain.commits[0].target.clone(),
        height: 1,
        kind: CheckpointRequestKind::Execution,
    };
    let bytes = encode_checkpoint_request(&request).unwrap();
    for field in 0..4 {
        let mut changed = bytes.clone();
        let offset = b"EVE_CHECKPOINT_REQUEST_V1".len();
        match field {
            0 => changed[offset] = 2,
            1 => changed[offset + 1] = 1,
            2 => changed[offset + 2] = 0,
            _ => changed.push(0),
        }
        let error = decode_checkpoint_request(&changed).unwrap_err();
        if field < 2 {
            assert_eq!(error, CheckpointMessageError::UnsupportedVersion);
        } else {
            assert_eq!(error, CheckpointMessageError::MalformedEncoding);
        }
    }
}
#[test]
fn invalid_height_width_index_and_non_genesis_anchor_refuse() {
    let (chain, _, _, _, id) = fixture();
    let mut request = CheckpointRequest {
        genesis: chain.commits[0].target.clone(),
        height: 1,
        kind: CheckpointRequestKind::Execution,
    };
    for height in [0, i64::MAX as u64 + 1] {
        request.height = height;
        assert!(encode_checkpoint_request(&request).is_err());
    }
    request.height = 1;
    for width in [0, 4_194_305] {
        request.kind = CheckpointRequestKind::Manifest { chunk_bytes: width };
        assert!(encode_checkpoint_request(&request).is_err());
    }
    request.kind = CheckpointRequestKind::Chunk {
        chunk_bytes: 64,
        manifest_id: id,
        index: 4_096,
    };
    assert!(encode_checkpoint_request(&request).is_err());
    request.kind = CheckpointRequestKind::Execution;
    request.genesis = chain.commits[1].target.clone();
    assert!(encode_checkpoint_request(&request).is_err());
}
#[test]
fn every_request_truncation_and_oversized_declared_genesis_refuse() {
    let (chain, _, _, _, _) = fixture();
    let request = CheckpointRequest {
        genesis: chain.commits[0].target.clone(),
        height: 1,
        kind: CheckpointRequestKind::Execution,
    };
    let bytes = encode_checkpoint_request(&request).unwrap();
    for length in 0..bytes.len() {
        assert!(decode_checkpoint_request(&bytes[..length]).is_err());
    }
    let mut changed = bytes;
    let length = b"EVE_CHECKPOINT_REQUEST_V1".len() + 3;
    changed[length..length + 4].copy_from_slice(&u32::MAX.to_be_bytes());
    assert!(decode_checkpoint_request(&changed).is_err());
    assert!(decode_checkpoint_request(&vec![0; 1_048_577]).is_err());
}
