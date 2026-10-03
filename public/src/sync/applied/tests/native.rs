// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use ed25519_dalek::{Signer, SigningKey};
use eve_consensus_comet::{
    consensus::{
        certificates::{
            ClassicalValidator, canonicalize_validator_set, hash_consensus_header,
            hash_transaction_data, hash_validator_set, validator_address,
        },
        signing::encode_vote_sign_bytes,
    },
    wire::tendermint::{
        types::{BlockId, Commit, CommitSig, Header, PartSetHeader, Vote},
        version::Consensus,
    },
};
use eve_finality_verifier::NativeFrame;
use eve_state::DevelopmentGenesis;

/// Actual canonical certificates signed by publicly known unsafe test identities.
pub(super) fn native_frame(
    genesis: &DevelopmentGenesis,
    height: i64,
    parent: Option<BlockId>,
    app: [u8; 32],
) -> NativeFrame {
    let validators = canonicalize_validator_set(
        &genesis
            .validators
            .iter()
            .map(|validator| ClassicalValidator {
                public_key: validator.classical_public_key,
                voting_power: validator.voting_power as i64,
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let set_hash = hash_validator_set(&validators).unwrap().to_vec();
    let mut header = Header {
        version: Some(Consensus { block: 11, app: 1 }),
        chain_id: genesis.network_name.clone(),
        height,
        time: Some(Default::default()),
        last_block_id: parent,
        data_hash: hash_transaction_data(&Vec::<Vec<u8>>::new())
            .unwrap()
            .to_vec(),
        validators_hash: set_hash.clone(),
        next_validators_hash: set_hash,
        consensus_hash: vec![0x43; 32],
        app_hash: app.to_vec(),
        proposer_address: validator_address(&validators[0].public_key).to_vec(),
        ..Default::default()
    };
    header.time.as_mut().unwrap().seconds = genesis.initial_timestamp as i64 + height;
    let block_id = BlockId {
        hash: hash_consensus_header(&header).unwrap().to_vec(),
        part_set_header: Some(PartSetHeader {
            total: 1,
            hash: vec![0x71; 32],
        }),
    };
    let signatures = validators
        .iter()
        .enumerate()
        .map(|(index, validator)| {
            let key = (1..=4_u8)
                .map(|seed| SigningKey::from_bytes(&[seed; 32]))
                .find(|key| key.verifying_key().to_bytes() == validator.public_key)
                .unwrap();
            let vote = Vote {
                r#type: 2,
                height,
                round: 0,
                block_id: Some(block_id.clone()),
                timestamp: header.time,
                validator_address: validator_address(&validator.public_key).to_vec(),
                validator_index: index as i32,
                ..Default::default()
            };
            CommitSig {
                block_id_flag: 2,
                validator_address: vote.validator_address.clone(),
                timestamp: vote.timestamp,
                signature: key
                    .sign(&encode_vote_sign_bytes(&header.chain_id, &vote).unwrap())
                    .to_bytes()
                    .to_vec(),
            }
        })
        .collect();
    let commit = Commit {
        height,
        round: 0,
        block_id: Some(block_id.clone()),
        signatures,
    };
    NativeFrame {
        block_id,
        header,
        commit,
    }
}
