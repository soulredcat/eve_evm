// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_development_fixtures::recovery::{RecoveryChain, recovery_chain};
use eve_finality_verifier::{
    CheckpointExecutionWitness, CheckpointLimits, CheckpointWitness, NativeDataFrame, NativeFrame,
};
use eve_state::{StateBudget, development_state_budget};

pub fn fixture() -> (RecoveryChain, StateBudget, CheckpointLimits) {
    (
        recovery_chain(),
        development_state_budget(),
        CheckpointLimits {
            maximum_height_gap: 8,
            maximum_witness_bytes: 13 * 1_048_576,
        },
    )
}
pub fn execution(chain: &RecoveryChain) -> CheckpointWitness {
    let frame = &chain.frames[0];
    CheckpointWitness::Execution(Box::new(CheckpointExecutionWitness {
        native: NativeDataFrame {
            frame: NativeFrame {
                block_id: frame.id.clone(),
                header: frame.header.clone(),
                commit: frame.commit.clone(),
            },
            transactions: chain.commits[1].block.transactions.clone(),
        },
        version: chain.commits[1].target.clone(),
        block: chain.commits[1].block.clone(),
    }))
}
pub fn lookahead(chain: &RecoveryChain) -> CheckpointWitness {
    let frame = &chain.frames[1];
    CheckpointWitness::Lookahead(Box::new(NativeDataFrame {
        frame: NativeFrame {
            block_id: frame.id.clone(),
            header: frame.header.clone(),
            commit: frame.commit.clone(),
        },
        transactions: chain.commits[2].block.transactions.clone(),
    }))
}
pub fn component_ranges(bytes: &[u8]) -> Vec<std::ops::Range<usize>> {
    let mut offset = b"EVE_CHECKPOINT_WITNESS_V1".len() + 1;
    let mut ranges = Vec::new();
    while offset < bytes.len() {
        let length = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        ranges.push(offset..offset + length);
        offset += length;
    }
    ranges
}
