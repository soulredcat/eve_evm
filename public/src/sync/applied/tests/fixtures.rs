// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::native::native_frame;
use crate::sync::applied::AppliedConfig;
use ed25519_dalek::SigningKey;
use eve_consensus_comet::consensus::certificates::validator_address;
use eve_evm::{ExecutionBlockInput, estimate_clone_reservation, execute_state_block};
use eve_finality_verifier::{
    CompactRecoveryEnvelopeV1, NativeDataFrame, NativeFrame, encode_compact_recovery_envelope,
};
use eve_state::{
    Address, B256, Bytes, DevelopmentGenesis, GenesisAccount, GenesisValidator, SecurityProfile,
    StateBudget, StateCommit, U256, development_economics, initialize_development_state,
};
use eve_storage::records::{OpaqueRecordIdentity, development_opaque_record_budget};
use std::path::Path;

pub(super) struct EmptyChain {
    pub(super) genesis: DevelopmentGenesis,
    pub(super) commits: Vec<StateCommit>,
    pub(super) records: Vec<Vec<u8>>,
}

pub(super) fn small_state_budget() -> StateBudget {
    StateBudget {
        maximum_accounts: 64,
        maximum_storage_slots: 128,
        maximum_codes: 16,
        maximum_code_bytes: 24_576,
        maximum_total_code_bytes: 65_536,
        maximum_system_records: 64,
        maximum_system_bytes: 32_768,
        maximum_block_hashes: 1_024,
        maximum_journal_operations: 512,
        maximum_journal_bytes: 65_536,
        maximum_state_bytes: 524_288,
        maximum_commit_bytes: 1_048_576,
    }
}

pub(super) fn genesis() -> DevelopmentGenesis {
    let economics = development_economics();
    DevelopmentGenesis {
        schema_version: 1,
        protocol_version: 1,
        network_name: "eve-local-v1".into(),
        evm_chain_id: 31_337,
        initial_timestamp: 1_728_000_000,
        profile: SecurityProfile::ClassicalDev,
        accounts: (1..=4_u8)
            .map(|seed| GenesisAccount {
                address: Address::repeat_byte(seed),
                funded_balance: economics.validator_self_bond * U256::from(10),
                nonce: 0,
                code: Bytes::new(),
            })
            .collect(),
        validators: (1..=4_u8)
            .map(|seed| GenesisValidator {
                owner: Address::repeat_byte(seed),
                classical_public_key: SigningKey::from_bytes(&[seed; 32])
                    .verifying_key()
                    .to_bytes(),
                self_bond: economics.validator_self_bond,
                voting_power: 10_000,
            })
            .collect(),
        economics,
        upgrades: Vec::new(),
    }
}

/// Independent canonical fixture preparation, with unsafe test-only validator identities.
pub(super) fn empty_chain() -> EmptyChain {
    let genesis = genesis();
    let budget = small_state_budget();
    let mut commits = vec![initialize_development_state(&genesis, &budget).unwrap()];
    let mut frames: Vec<NativeFrame> = Vec::new();
    for height in 1..=3 {
        let parent = commits.last().unwrap();
        let app = parent
            .target
            .application
            .map_or(parent.target.content_digest.0, |app| app.0.0);
        let frame = native_frame(
            &genesis,
            height,
            frames.last().map(|previous| previous.block_id.clone()),
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
            timestamp: frame.header.time.unwrap().seconds as u64,
            proposer,
            previous_consensus_hash: frame
                .header
                .last_block_id
                .as_ref()
                .map_or(B256::ZERO, |id| B256::from_slice(&id.hash)),
        };
        let oracle = estimate_clone_reservation(&parent.state).unwrap();
        let prepared = execute_state_block(parent, &input, &[], &budget, oracle).unwrap();
        commits.push(prepared.commit);
        frames.push(frame);
    }
    let records = (1..=2)
        .map(|height| {
            let envelope = CompactRecoveryEnvelopeV1 {
                parent: commits[height - 1].target.clone(),
                expected: commits[height].target.clone(),
                execution: commits[height].block.clone(),
                finalized: frames[height - 1].clone(),
                lookahead: NativeDataFrame {
                    frame: frames[height].clone(),
                    transactions: Vec::new(),
                },
            };
            encode_compact_recovery_envelope(&envelope, &budget).unwrap()
        })
        .collect();
    EmptyChain {
        genesis,
        commits,
        records,
    }
}

pub(super) fn config(path: &Path, chain: &EmptyChain) -> AppliedConfig {
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
        state_budget: small_state_budget(),
        repository_budget,
        worker_scratch_limit: 40 * 1_048_576,
        maximum_recovery_payload_bytes: 65_536,
    }
}
