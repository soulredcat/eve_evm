// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{Frame, seeded_signing_key::seeded_signing_key};
use ed25519_dalek::Signer;
use eve_consensus_comet::{
    consensus::{
        certificates::{ClassicalValidator, validator_address},
        signing::encode_vote_sign_bytes,
    },
    wire::tendermint::types::{CommitSig, Vote},
};

pub(super) fn sign_commit_vote(
    frame: &Frame,
    index: usize,
    validator: &ClassicalValidator,
) -> CommitSig {
    let key = seeded_signing_key(&validator.public_key);
    let vote = Vote {
        r#type: 2,
        height: frame.header.height,
        round: 0,
        block_id: Some(frame.id.clone()),
        timestamp: frame.header.time,
        validator_address: validator_address(&validator.public_key).to_vec(),
        validator_index: i32::try_from(index).unwrap(),
        ..Default::default()
    };
    CommitSig {
        block_id_flag: 2,
        validator_address: vote.validator_address.clone(),
        timestamp: vote.timestamp,
        signature: key
            .sign(&encode_vote_sign_bytes(&frame.header.chain_id, &vote).unwrap())
            .to_bytes()
            .to_vec(),
    }
}
