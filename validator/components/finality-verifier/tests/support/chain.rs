// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    genesis,
    native::{self, Frame},
};
use eve_consensus_comet::consensus::certificates::validator_address;
use eve_evm::{ExecutionBlockInput, execute_state_block};
use eve_finality_verifier::{
    DevelopmentFinalityVerifier, FinalityError, initialize_development_finality,
    verify_next_development_header,
};
use eve_state::{
    B256, DevelopmentGenesis, StateCommit, development_state_budget, initialize_development_state,
};

pub struct Chain {
    pub genesis: DevelopmentGenesis,
    pub commits: Vec<StateCommit>,
    pub frames: Vec<Frame>,
}

pub fn chain() -> Chain {
    let genesis = genesis::genesis();
    let initial = initialize_development_state(&genesis, &development_state_budget()).unwrap();
    let mut commits = vec![initial];
    let mut frames = Vec::new();
    for height in 1..=3 {
        let parent = commits.last().unwrap();
        let app = if height == 1 {
            parent.target.content_digest.0
        } else {
            parent.target.application.unwrap().0.0
        };
        let frame = native::frame(
            &genesis,
            height,
            frames.last().map(|previous: &Frame| previous.id.clone()),
            app,
        );
        let proposer = genesis
            .validators
            .iter()
            .find(|validator| {
                validator_address(&validator.classical_public_key).as_slice()
                    == frame.header.proposer_address
            })
            .unwrap()
            .owner;
        let input = ExecutionBlockInput {
            timestamp: u64::try_from(frame.header.time.unwrap().seconds).unwrap(),
            proposer,
            previous_consensus_hash: frame
                .header
                .last_block_id
                .as_ref()
                .map_or(B256::ZERO, |id| B256::from_slice(&id.hash)),
        };
        let prepared = execute_state_block(
            parent,
            &input,
            &[],
            &development_state_budget(),
            512 * 1_048_576,
        )
        .unwrap();
        commits.push(prepared.commit);
        frames.push(frame);
    }
    Chain {
        genesis,
        commits,
        frames,
    }
}

pub fn verifier(genesis: &DevelopmentGenesis) -> DevelopmentFinalityVerifier {
    initialize_development_finality(genesis, &development_state_budget()).unwrap()
}

pub fn accept(verifier: &mut DevelopmentFinalityVerifier, frame: &Frame) {
    let verified = verify_next_development_header(
        verifier,
        &frame.id,
        &frame.header,
        &frame.commit,
        &frame.validators,
        &[],
    )
    .unwrap();
    assert_eq!(verified.identity(), verifier.identity());
    assert_eq!(verified.native().header(), &frame.header);
    assert_eq!(verified.native().block_id(), &frame.id);
    assert_eq!(verifier.height(), frame.header.height);
}

pub fn reject(verifier: &mut DevelopmentFinalityVerifier, frame: &Frame) -> FinalityError {
    let before = (verifier.identity().clone(), verifier.height());
    let error = verify_next_development_header(
        verifier,
        &frame.id,
        &frame.header,
        &frame.commit,
        &frame.validators,
        &[],
    )
    .err()
    .unwrap();
    assert_eq!((verifier.identity().clone(), verifier.height()), before);
    error
}
