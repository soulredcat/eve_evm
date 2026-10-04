// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_development_fixtures::recovery::{RecoveryChain, recovery_chain};
use eve_finality_verifier::{
    AuthenticatedImportInput, NativeDataFrame, NativeFrame, encode_logical_import_wire,
};
use eve_state::{development_state_budget, project_state_journal};
use eve_storage::records::{
    OpaqueCompactionRequest, OpaqueRecordBudget, OpaqueRecordIdentity, OpaqueRecordRepository,
    development_opaque_record_budget, opaque_record_budget, opaque_record_cursor,
    opaque_record_identity, required_opaque_compaction_reservation,
};

pub struct Fixture {
    pub chain: RecoveryChain,
    pub wires: Vec<Vec<u8>>,
}

pub fn fixture() -> Fixture {
    let chain = recovery_chain();
    let budget = development_state_budget();
    let mut wires = Vec::new();
    for height in 1..=2 {
        let finalized = &chain.frames[height - 1];
        let lookahead = &chain.frames[height];
        let input = AuthenticatedImportInput {
            journal: project_state_journal(
                &chain.commits[height - 1].state,
                &chain.commits[height - 1].target,
                &chain.commits[height].state,
                height as u64,
                &budget,
            )
            .unwrap(),
            execution: chain.commits[height].block.clone(),
            finalized: NativeFrame {
                block_id: finalized.id.clone(),
                header: finalized.header.clone(),
                commit: finalized.commit.clone(),
            },
            lookahead: NativeDataFrame {
                frame: NativeFrame {
                    block_id: lookahead.id.clone(),
                    header: lookahead.header.clone(),
                    commit: lookahead.commit.clone(),
                },
                transactions: chain.commits[height + 1].block.transactions.clone(),
            },
        };
        wires.push(encode_logical_import_wire(&input, &budget).unwrap());
    }
    Fixture { chain, wires }
}

pub fn identity() -> OpaqueRecordIdentity {
    // Structural local namespace only; signatures come from canonical development fixtures.
    OpaqueRecordIdentity {
        genesis_hash: [1; 32],
        owner: [2; 32],
        domain: [3; 32],
    }
}

pub fn budget() -> OpaqueRecordBudget {
    OpaqueRecordBudget {
        maximum_record_bytes: 1_048_576,
        maximum_read_bytes: 1_048_576,
        maximum_batch_bytes: 2_097_152,
        maximum_batch_records: 2,
        maximum_retained_records: 2,
        block_cache_bytes: 65_536,
        write_buffer_bytes: 65_536,
        ..development_opaque_record_budget()
    }
}

pub fn request(repository: &OpaqueRecordRepository) -> OpaqueCompactionRequest {
    OpaqueCompactionRequest {
        expected_identity: opaque_record_identity(repository),
        expected_head: opaque_record_cursor(repository).unwrap(),
        maximum_records: 2,
    }
}

pub fn charge(repository: &OpaqueRecordRepository) -> usize {
    required_opaque_compaction_reservation(&opaque_record_budget(repository)).unwrap()
}
