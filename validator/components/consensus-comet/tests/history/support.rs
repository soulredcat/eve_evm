// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use ed25519_dalek::{Signer, SigningKey};
use eve_consensus_comet::{
    consensus::{
        authentication::ConsensusAuthenticationRequirement,
        certificates::{
            ClassicalValidator, canonicalize_validator_set, hash_consensus_header,
            hash_transaction_data, hash_validator_set, validator_address,
        },
        history::{NativeHistoryVerifier, initialize_genesis_history},
        signing::encode_vote_sign_bytes,
    },
    wire::tendermint::{
        types::{BlockId, Commit, CommitSig, Header, PartSetHeader, Vote},
        version::Consensus,
    },
};

pub const CHAIN: &str = "eve-local-v1";
pub const GENESIS_APP: [u8; 32] = [0x47; 32];

/// Unsafe deterministic keys exist only in these local-genesis trust fixtures.
pub struct Roster {
    pub validators: Vec<ClassicalValidator>,
    keys: Vec<SigningKey>,
}

impl Roster {
    pub fn new(first_seed: u8, count: u8) -> Self {
        let keys: Vec<_> = (first_seed..first_seed + count)
            .map(|seed| SigningKey::from_bytes(&[seed; 32]))
            .collect();
        let validators = canonicalize_validator_set(
            &keys
                .iter()
                .map(|key| ClassicalValidator {
                    public_key: key.verifying_key().to_bytes(),
                    voting_power: 1,
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
        Self { validators, keys }
    }

    pub fn history(&self) -> NativeHistoryVerifier {
        initialize_genesis_history(
            CHAIN,
            &self.validators,
            GENESIS_APP,
            ConsensusAuthenticationRequirement::ClassicalDev,
        )
        .unwrap()
    }

    pub fn frame(&self, height: i64, parent: Option<BlockId>, next: &Roster) -> Frame {
        let transactions = vec![vec![0x91, u8::try_from(height).unwrap()]];
        let header = Header {
            version: Some(Consensus { block: 11, app: 1 }),
            chain_id: CHAIN.into(),
            height,
            time: Some(prost_types::Timestamp {
                seconds: 1_700_000_000 + height,
                nanos: 0,
            }),
            last_block_id: parent,
            last_commit_hash: Vec::new(),
            data_hash: hash_transaction_data(&transactions).unwrap().to_vec(),
            validators_hash: hash_validator_set(&self.validators).unwrap().to_vec(),
            next_validators_hash: hash_validator_set(&next.validators).unwrap().to_vec(),
            consensus_hash: vec![0x43; 32],
            app_hash: if height == 1 {
                GENESIS_APP.to_vec()
            } else {
                vec![0x50; 32]
            },
            last_results_hash: Vec::new(),
            evidence_hash: Vec::new(),
            proposer_address: validator_address(&self.validators[0].public_key).to_vec(),
        };
        let mut frame = Frame {
            header,
            block_id: BlockId::default(),
            commit: Commit::default(),
            transactions,
        };
        self.resign(&mut frame);
        frame
    }

    pub fn resign(&self, frame: &mut Frame) {
        frame.block_id = BlockId {
            hash: hash_consensus_header(&frame.header).unwrap().to_vec(),
            part_set_header: Some(PartSetHeader {
                total: 1,
                hash: vec![0x71; 32],
            }),
        };
        let signatures = self
            .validators
            .iter()
            .enumerate()
            .map(|(index, validator)| {
                let key = self
                    .keys
                    .iter()
                    .find(|key| key.verifying_key().to_bytes() == validator.public_key)
                    .unwrap();
                let vote = Vote {
                    r#type: 2,
                    height: frame.header.height,
                    round: 0,
                    block_id: Some(frame.block_id.clone()),
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
            block_id: Some(frame.block_id.clone()),
            signatures,
        };
    }
}

pub struct Frame {
    pub header: Header,
    pub block_id: BlockId,
    pub commit: Commit,
    pub transactions: Vec<Vec<u8>>,
}

#[path = "assertions.rs"]
mod assertions;
pub use assertions::{accept, reject_unchanged};
