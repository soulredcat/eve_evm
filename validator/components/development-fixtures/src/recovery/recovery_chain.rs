// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CLONE_BYTES, RecoveryChain, copy_previous_block_id::copy_previous_block_id, funded_genesis,
    recovery_execution_input::recovery_execution_input, signed_transaction,
};
use crate::native;
use eve_consensus_comet::consensus::certificates::hash_transaction_data;
use eve_evm::execute_state_block;
use eve_state::{development_state_budget, initialize_development_state};

pub fn recovery_chain() -> RecoveryChain {
    let genesis = funded_genesis();
    let initial = initialize_development_state(&genesis, &development_state_budget()).unwrap();
    let mut commits = vec![initial];
    let mut frames: Vec<native::Frame> = Vec::new();
    for height in 1..=3 {
        let parent = commits.last().unwrap();
        let application = if height == 1 {
            parent.target.content_digest.0
        } else {
            parent.target.application.unwrap().0.0
        };
        let previous = frames.last().map(copy_previous_block_id);
        let mut frame = native::frame(&genesis, height, previous, application);
        let transactions = if height == 1 {
            vec![signed_transaction()]
        } else {
            Vec::new()
        };
        frame.header.data_hash = hash_transaction_data(&transactions).unwrap().to_vec();
        native::resign(&mut frame);
        let prepared = execute_state_block(
            parent,
            &recovery_execution_input(&genesis, &frame),
            &transactions,
            &development_state_budget(),
            CLONE_BYTES,
        )
        .unwrap();
        commits.push(prepared.commit);
        frames.push(frame);
    }
    RecoveryChain {
        genesis,
        commits,
        frames,
    }
}
