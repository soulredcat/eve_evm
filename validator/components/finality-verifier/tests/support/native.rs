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
use eve_state::DevelopmentGenesis;

pub struct Frame {
    pub header: Header,
    pub id: BlockId,
    pub commit: Commit,
    pub validators: Vec<ClassicalValidator>,
}

pub fn frame(
    genesis: &DevelopmentGenesis,
    height: i64,
    parent: Option<BlockId>,
    app: [u8; 32],
) -> Frame {
    let validators = canonicalize_validator_set(
        &genesis
            .validators
            .iter()
            .map(|validator| ClassicalValidator {
                public_key: validator.classical_public_key,
                voting_power: i64::try_from(validator.voting_power).unwrap(),
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let set_hash = hash_validator_set(&validators).unwrap().to_vec();
    let header = Header {
        version: Some(Consensus {
            block: 11,
            app: u64::from(genesis.protocol_version),
        }),
        chain_id: genesis.network_name.clone(),
        height,
        time: Some(prost_types::Timestamp {
            seconds: i64::try_from(genesis.initial_timestamp).unwrap() + height,
            nanos: 0,
        }),
        last_block_id: parent,
        last_commit_hash: Vec::new(),
        data_hash: hash_transaction_data(&Vec::<Vec<u8>>::new())
            .unwrap()
            .to_vec(),
        validators_hash: set_hash.clone(),
        next_validators_hash: set_hash,
        consensus_hash: vec![0x43; 32],
        app_hash: app.to_vec(),
        last_results_hash: Vec::new(),
        evidence_hash: Vec::new(),
        proposer_address: validator_address(&validators[0].public_key).to_vec(),
    };
    let mut frame = Frame {
        header,
        id: BlockId::default(),
        commit: Commit::default(),
        validators,
    };
    resign(&mut frame);
    frame
}

pub fn resign(frame: &mut Frame) {
    frame.id = BlockId {
        hash: hash_consensus_header(&frame.header).unwrap().to_vec(),
        part_set_header: Some(PartSetHeader {
            total: 1,
            hash: vec![0x71; 32],
        }),
    };
    let signatures = frame
        .validators
        .iter()
        .enumerate()
        .map(|(index, validator)| {
            let key = (1..=4_u8)
                .map(|seed| SigningKey::from_bytes(&[seed; 32]))
                .find(|key| key.verifying_key().to_bytes() == validator.public_key)
                .unwrap();
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
        })
        .collect();
    frame.commit = Commit {
        height: frame.header.height,
        round: 0,
        block_id: Some(frame.id.clone()),
        signatures,
    };
}
