// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery_support::{RecoveryChain, native_frame};
use eve_finality_verifier::{
    CheckpointExecutionWitness, CheckpointLimits, CheckpointSession, CheckpointWitness,
    ImportedState, begin_authenticated_checkpoint, required_checkpoint_reservation,
};
use eve_state::development_state_budget;
use std::sync::Arc;

pub fn limits() -> CheckpointLimits {
    CheckpointLimits {
        maximum_height_gap: 8,
        maximum_witness_bytes: 13 * 1_048_576,
    }
}
pub fn execution(chain: &RecoveryChain, height: usize) -> CheckpointWitness {
    CheckpointWitness::Execution(Box::new(CheckpointExecutionWitness {
        native: eve_finality_verifier::NativeDataFrame {
            frame: native_frame(&chain.frames[height - 1]),
            transactions: chain.commits[height].block.transactions.clone(),
        },
        version: chain.commits[height].target.clone(),
        block: chain.commits[height].block.clone(),
    }))
}
pub fn lookahead(chain: &RecoveryChain) -> CheckpointWitness {
    CheckpointWitness::Lookahead(Box::new(eve_finality_verifier::NativeDataFrame {
        frame: native_frame(&chain.frames[2]),
        transactions: chain.commits[3].block.transactions.clone(),
    }))
}
pub fn session(parent: &ImportedState, chain: &RecoveryChain) -> (CheckpointSession, usize) {
    let budget = development_state_budget();
    let required =
        required_checkpoint_reservation(parent, &chain.commits[2], &budget, limits()).unwrap();
    (
        begin_authenticated_checkpoint(
            parent,
            Arc::new(chain.commits[2].clone()),
            &budget,
            limits(),
            required,
        )
        .unwrap(),
        required,
    )
}
