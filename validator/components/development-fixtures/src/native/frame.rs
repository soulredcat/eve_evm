// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::Frame;
use eve_consensus_comet::{
    consensus::certificates::{
        ClassicalValidator, canonicalize_validator_set, hash_transaction_data, hash_validator_set,
        validator_address,
    },
    wire::tendermint::{
        types::{BlockId, Commit, Header},
        version::Consensus,
    },
};
use eve_state::DevelopmentGenesis;

pub fn frame(
    genesis: &DevelopmentGenesis,
    height: i64,
    parent: Option<BlockId>,
    app: [u8; 32],
) -> Frame {
    let mut enrolled = Vec::new();
    for validator in &genesis.validators {
        enrolled.push(ClassicalValidator {
            public_key: validator.classical_public_key,
            voting_power: i64::try_from(validator.voting_power).unwrap(),
        });
    }
    let validators = canonicalize_validator_set(&enrolled).unwrap();
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
    super::resign(&mut frame);
    frame
}
