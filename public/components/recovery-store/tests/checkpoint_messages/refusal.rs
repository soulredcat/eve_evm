// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::fixture;
use eve_state::Bytes;
use eve_storage::checkpoints::messages::{
    CheckpointMessageError, CheckpointResponse, decode_checkpoint_response,
    encode_checkpoint_response, preflight_checkpoint_response,
    required_checkpoint_response_decode_reservation,
};
#[test]
fn response_truncation_trailing_compression_and_unsupported_version_refuse() {
    let (chain, limits, _, manifest, id) = fixture();
    let response = CheckpointResponse::Manifest {
        target: chain.commits[1].target.clone(),
        durable_tip: Box::new(chain.commits[2].target.clone()),
        manifest_id: id,
        manifest,
    };
    let bytes = encode_checkpoint_response(&response, &limits).unwrap();
    for length in 0..bytes.len() {
        assert!(preflight_checkpoint_response(&bytes[..length], &limits).is_err());
    }
    for index in [0, 1, 2, 3] {
        let mut changed = bytes.clone();
        let domain = b"EVE_CHECKPOINT_RESPONSE_V1".len();
        match index {
            0 => changed[domain] = 2,
            1 => changed[domain + 1] = 1,
            2 => changed[domain + 2] = 0,
            _ => changed.push(0),
        }
        assert!(preflight_checkpoint_response(&changed, &limits).is_err());
    }
}
#[test]
fn mismatched_manifest_chunk_bounds_limits_and_insufficient_decode_reservation_refuse() {
    let (chain, mut limits, body, manifest, _) = fixture();
    let manifest_response = CheckpointResponse::Manifest {
        target: chain.commits[1].target.clone(),
        durable_tip: Box::new(chain.commits[2].target.clone()),
        manifest_id: [0; 32],
        manifest,
    };
    assert!(encode_checkpoint_response(&manifest_response, &limits).is_err());
    for (index, data) in [(0, vec![0; 63]), (4_096, vec![0; 64])] {
        let response = CheckpointResponse::Chunk {
            target: chain.commits[1].target.clone(),
            manifest_id: [1; 32],
            body_sha256: [2; 32],
            total_length: body.len() as u64,
            index,
            chunk_bytes: 64,
            data: Bytes::from(data),
        };
        assert!(encode_checkpoint_response(&response, &limits).is_err());
    }
    let response = CheckpointResponse::Execution {
        target: chain.commits[1].target.clone(),
        block: Box::new(chain.commits[1].block.clone()),
    };
    let bytes = encode_checkpoint_response(&response, &limits).unwrap();
    let preflight = preflight_checkpoint_response(&bytes, &limits).unwrap();
    let required = required_checkpoint_response_decode_reservation(&preflight).unwrap();
    assert_eq!(
        decode_checkpoint_response(&preflight, required - 1).err(),
        Some(CheckpointMessageError::ReservationTooSmall)
    );
    limits.maximum_body_bytes = 32 * 1_048_576 + 1;
    assert!(preflight_checkpoint_response(&bytes, &limits).is_err());
}
#[test]
fn malformed_block_is_rejected_by_maintained_decoder_after_bounded_lease_admission() {
    let (chain, limits, _, _, _) = fixture();
    let response = CheckpointResponse::Execution {
        target: chain.commits[1].target.clone(),
        block: Box::new(chain.commits[1].block.clone()),
    };
    let mut bytes = encode_checkpoint_response(&response, &limits).unwrap();
    let mut offset = b"EVE_CHECKPOINT_RESPONSE_V1".len() + 3;
    let version = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
    offset += 4 + version;
    let block = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
    offset += 4;
    bytes[offset..offset + block].fill(0);
    let preflight = preflight_checkpoint_response(&bytes, &limits).unwrap();
    let reserved = required_checkpoint_response_decode_reservation(&preflight).unwrap();
    assert!(matches!(
        decode_checkpoint_response(&preflight, reserved),
        Err(CheckpointMessageError::State(_))
    ));
}
