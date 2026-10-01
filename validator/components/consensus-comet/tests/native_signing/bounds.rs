// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{proposal, vote};
use eve_consensus_comet::consensus::signing::{
    SigningError, encode_proposal_sign_bytes, encode_vote_sign_bytes,
};

#[test]
fn rejects_invalid_chain_height_round_and_message_types_before_signing() {
    for chain in ["", "bad\nchain", &"x".repeat(51)] {
        assert_eq!(
            encode_vote_sign_bytes(chain, &vote()),
            Err(SigningError::InvalidChainId)
        );
        assert_eq!(
            encode_proposal_sign_bytes(chain, &proposal()),
            Err(SigningError::InvalidChainId)
        );
    }
    for height in [0, -1, i64::MIN] {
        let mut candidate = vote();
        candidate.height = height;
        assert_eq!(
            encode_vote_sign_bytes("eve-local-v1", &candidate),
            Err(SigningError::InvalidHeight)
        );
    }
    for kind in [0, 3, 32, i32::MAX] {
        let mut candidate = vote();
        candidate.r#type = kind;
        assert_eq!(
            encode_vote_sign_bytes("eve-local-v1", &candidate),
            Err(SigningError::InvalidMessageType)
        );
    }
    let mut candidate = vote();
    candidate.round = -1;
    assert_eq!(
        encode_vote_sign_bytes("eve-local-v1", &candidate),
        Err(SigningError::InvalidRound)
    );
}

#[test]
fn rejects_invalid_pol_round_and_empty_proposal_block_id() {
    for pol_round in [-2, 2, 3, i32::MAX] {
        let mut candidate = proposal();
        candidate.pol_round = pol_round;
        assert_eq!(
            encode_proposal_sign_bytes("eve-local-v1", &candidate),
            Err(SigningError::InvalidRound)
        );
    }
    let mut candidate = proposal();
    candidate.block_id = None;
    assert_eq!(
        encode_proposal_sign_bytes("eve-local-v1", &candidate),
        Err(SigningError::InvalidBlockId)
    );
    let mut candidate = proposal();
    candidate.r#type = 1;
    assert_eq!(
        encode_proposal_sign_bytes("eve-local-v1", &candidate),
        Err(SigningError::InvalidMessageType)
    );
}

#[test]
fn rejects_partial_ids_invalid_hash_width_and_native_routing_bounds() {
    for size in [0, 1, 31, 33, 4096] {
        let mut candidate = vote();
        candidate.block_id.as_mut().unwrap().hash = vec![1; size];
        assert_eq!(
            encode_vote_sign_bytes("eve-local-v1", &candidate),
            Err(SigningError::InvalidBlockId)
        );
    }
    let mut candidate = vote();
    candidate
        .block_id
        .as_mut()
        .unwrap()
        .part_set_header
        .as_mut()
        .unwrap()
        .total = 0;
    assert_eq!(
        encode_vote_sign_bytes("eve-local-v1", &candidate),
        Err(SigningError::InvalidBlockId)
    );
    for size in [0, 19, 21, 4096] {
        let mut candidate = vote();
        candidate.validator_address = vec![1; size];
        assert_eq!(
            encode_vote_sign_bytes("eve-local-v1", &candidate),
            Err(SigningError::InvalidValidatorAddress)
        );
    }
    let mut candidate = vote();
    candidate.validator_index = -1;
    assert_eq!(
        encode_vote_sign_bytes("eve-local-v1", &candidate),
        Err(SigningError::InvalidValidatorIndex)
    );
}

#[test]
fn rejects_invalid_timestamp_signature_width_and_vote_extensions() {
    for (seconds, nanos) in [
        (-62_135_596_801, 0),
        (253_402_300_800, 0),
        (0, -1),
        (0, 1_000_000_000),
    ] {
        let mut candidate = vote();
        candidate.timestamp = Some(prost_types::Timestamp { seconds, nanos });
        assert_eq!(
            encode_vote_sign_bytes("eve-local-v1", &candidate),
            Err(SigningError::InvalidTimestamp)
        );
    }
    for size in [1, 63, 65, 4096] {
        let mut candidate = vote();
        candidate.signature = vec![1; size];
        assert_eq!(
            encode_vote_sign_bytes("eve-local-v1", &candidate),
            Err(SigningError::InvalidSignatureLength)
        );
    }
    let mut candidate = vote();
    candidate.extension = vec![1];
    assert_eq!(
        encode_vote_sign_bytes("eve-local-v1", &candidate),
        Err(SigningError::UnsupportedVoteExtension)
    );
    let mut candidate = vote();
    candidate.extension_signature = vec![1; 64];
    assert_eq!(
        encode_vote_sign_bytes("eve-local-v1", &candidate),
        Err(SigningError::UnsupportedVoteExtension)
    );
}
