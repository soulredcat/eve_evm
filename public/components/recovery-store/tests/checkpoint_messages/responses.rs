// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::fixture;
use eve_state::Bytes;
use eve_storage::checkpoints::{
    checkpoint_manifest_stats,
    messages::{
        CheckpointResponse, checkpoint_message_storage_limits, checkpoint_response_bytes,
        checkpoint_response_stats, decode_checkpoint_response, encode_checkpoint_response,
        preflight_checkpoint_response, required_checkpoint_response_decode_reservation,
    },
    preflight_checkpoint_manifest,
};
#[test]
fn all_responses_roundtrip_with_exact_content_and_execution_metadata_without_certificates() {
    let (chain, limits, body, manifest, id) = fixture();
    let target = chain.commits[1].target.clone();
    let version = eve_state::encode_state_version(&target).unwrap();
    let storage = checkpoint_message_storage_limits(&limits, 64).unwrap();
    let stats = checkpoint_manifest_stats(
        &preflight_checkpoint_manifest(&manifest, &version, &storage).unwrap(),
    );
    let responses = [
        CheckpointResponse::Manifest {
            target: target.clone(),
            durable_tip: Box::new(chain.commits[2].target.clone()),
            manifest_id: id,
            manifest,
        },
        CheckpointResponse::Chunk {
            target: target.clone(),
            manifest_id: id,
            body_sha256: stats.body_sha256,
            total_length: body.len() as u64,
            index: 0,
            chunk_bytes: 64,
            data: Bytes::copy_from_slice(&body[..64]),
        },
        CheckpointResponse::Execution {
            target,
            block: Box::new(chain.commits[1].block.clone()),
        },
    ];
    for response in responses {
        let bytes = encode_checkpoint_response(&response, &limits).unwrap();
        let preflight = preflight_checkpoint_response(&bytes, &limits).unwrap();
        assert_eq!(
            checkpoint_response_bytes(&preflight).as_ptr(),
            bytes.as_ptr()
        );
        assert_eq!(
            checkpoint_response_stats(&preflight).encoded_bytes,
            bytes.len()
        );
        let required = required_checkpoint_response_decode_reservation(&preflight).unwrap();
        let decoded = decode_checkpoint_response(&preflight, required).unwrap();
        assert_eq!(decoded, response);
        assert_eq!(
            encode_checkpoint_response(&decoded, &limits).unwrap(),
            bytes
        );
    }
}
#[test]
fn final_chunk_has_exact_short_tail_and_sealed_policy_cannot_be_rebound() {
    let (chain, mut limits, body, _, id) = fixture();
    let width = 64;
    let index = body.len().div_ceil(width) - 1;
    let response = CheckpointResponse::Chunk {
        target: chain.commits[1].target.clone(),
        manifest_id: id,
        body_sha256: [9; 32],
        total_length: body.len() as u64,
        index: index as u32,
        chunk_bytes: width as u32,
        data: Bytes::copy_from_slice(&body[index * width..]),
    };
    let bytes = encode_checkpoint_response(&response, &limits).unwrap();
    let preflight = preflight_checkpoint_response(&bytes, &limits).unwrap();
    limits.maximum_chunk_bytes = 1;
    let required = required_checkpoint_response_decode_reservation(&preflight).unwrap();
    assert_eq!(
        decode_checkpoint_response(&preflight, required).unwrap(),
        response
    );
    assert!(preflight_checkpoint_response(&bytes, &limits).is_err());
}
