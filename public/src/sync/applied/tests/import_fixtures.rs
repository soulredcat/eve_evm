// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{fixtures::genesis, native::native_frame_with_transactions};
use crate::sync::applied::AppliedConfig;
use eve_consensus_comet::consensus::certificates::validator_address;
use eve_evm::{
    ExecutionBlockInput, decode_signed_transaction, estimate_clone_reservation, execute_state_block,
};
use eve_finality_verifier::{
    AuthenticatedImportInput, NativeDataFrame, NativeFrame, encode_authenticated_import_wire,
};
use eve_state::{
    Address, B256, Bytes, DevelopmentGenesis, GenesisAccount, StateCommit, U256,
    development_state_budget, initialize_development_state,
};
use eve_storage::records::{OpaqueRecordIdentity, development_opaque_record_budget};
use std::path::Path;

pub(crate) struct ImportChain {
    pub(crate) genesis: DevelopmentGenesis,
    pub(crate) commits: Vec<StateCommit>,
    pub(crate) inputs: Vec<AuthenticatedImportInput>,
    pub(crate) records: Vec<Vec<u8>>,
}

/// Existing public test vector: native chain31337, nonce0, contract slot write.
pub(crate) fn signed_transaction() -> Bytes {
    "0xf866808477359400830186a0940000000000000000000000000000000000000042808082f4f5a02c1d1a7db8b28ed638c4c0a70b8789ae4fc8d41fbf881cca9ccb0a97f8f487aba00a5013ea38ccecec4dab4c2c82674109c7a303ad69772b56ef82a47eef05d4c6".parse().unwrap()
}

/// A canonical replay oracle creates the delta; import is tested without re-execution.
pub(crate) fn import_chain() -> ImportChain {
    let budget = development_state_budget();
    let mut genesis = genesis();
    let sender = decode_signed_transaction(&signed_transaction(), 31_337, 131_072)
        .unwrap()
        .sender();
    genesis.accounts.push(GenesisAccount {
        address: sender,
        funded_balance: U256::from(10_u64).pow(U256::from(22)),
        nonce: 0,
        code: Bytes::new(),
    });
    genesis.accounts.push(GenesisAccount {
        address: Address::with_last_byte(0x42),
        funded_balance: U256::ZERO,
        nonce: 0,
        code: "0x606360005500".parse().unwrap(),
    });
    let mut commits = vec![initialize_development_state(&genesis, &budget).unwrap()];
    let mut frames: Vec<NativeFrame> = Vec::new();
    let mut journals = Vec::new();
    for height in 1..=3 {
        let parent = commits.last().unwrap();
        let transactions = if height == 1 {
            vec![signed_transaction()]
        } else {
            Vec::new()
        };
        let app = parent
            .target
            .application
            .map_or(parent.target.content_digest.0, |app| app.0.0);
        let frame = native_frame_with_transactions(
            &genesis,
            height,
            frames.last().map(|previous| previous.block_id.clone()),
            app,
            &transactions,
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
            timestamp: frame.header.time.unwrap().seconds as u64,
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
            &transactions,
            &budget,
            estimate_clone_reservation(&parent.state).unwrap(),
        )
        .unwrap();
        journals.push(prepared.journal);
        commits.push(prepared.commit);
        frames.push(frame);
    }
    let inputs: Vec<_> = (1..=2)
        .map(|height| AuthenticatedImportInput {
            journal: journals[height - 1].clone(),
            execution: commits[height].block.clone(),
            finalized: frames[height - 1].clone(),
            lookahead: NativeDataFrame {
                frame: frames[height].clone(),
                transactions: commits[height + 1].block.transactions.clone(),
            },
        })
        .collect();
    let records = inputs
        .iter()
        .map(|input| encode_authenticated_import_wire(input, &budget).unwrap())
        .collect();
    ImportChain {
        genesis,
        commits,
        inputs,
        records,
    }
}

pub(crate) fn import_config(path: &Path, chain: &ImportChain) -> AppliedConfig {
    let mut repository_budget = development_opaque_record_budget();
    repository_budget.maximum_open_files = 32;
    repository_budget.maximum_record_bytes = 65_536 + 88;
    repository_budget.maximum_read_bytes = 65_536 + 88;
    AppliedConfig {
        path: path.to_owned(),
        identity: OpaqueRecordIdentity {
            genesis_hash: chain.commits[0].target.identity.genesis.0.0,
            owner: [0x51; 32],
            domain: [0x52; 32],
        },
        public_budget: eve_node_policy::development_public_budget(),
        state_budget: development_state_budget(),
        repository_budget,
        worker_scratch_limit: 40 * 1_048_576,
        maximum_recovery_payload_bytes: 65_536,
    }
}
