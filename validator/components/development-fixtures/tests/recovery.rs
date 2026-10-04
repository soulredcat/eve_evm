// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    certificates::{
        HistoricalValidatorSet, hash_transaction_data, validator_address, verify_commit_certificate,
    },
};
use eve_development_fixtures::recovery::{CLONE_BYTES, recovery_chain, sender, signed_transaction};
use eve_evm::{ExecutionBlockInput, execute_state_block};
use eve_state::{
    Address, B256, U256, development_state_budget, initialize_development_state,
    validate_state_commit,
};

#[test]
fn three_height_chain_keeps_exact_serial_execution_roots_receipts_and_native_context() {
    let chain = recovery_chain();
    let budget = development_state_budget();
    assert_eq!((chain.commits.len(), chain.frames.len()), (4, 3));
    let mut reference = initialize_development_state(&chain.genesis, &budget).unwrap();
    assert_eq!(reference, chain.commits[0]);
    for height in 1..=3 {
        let native = &chain.frames[height - 1];
        let actual = &chain.commits[height];
        let native_height = i64::try_from(height).unwrap();
        let set = HistoricalValidatorSet {
            height: native_height,
            authentication: ConsensusAuthenticationRequirement::ClassicalDev,
            validators: native.validators.clone(),
        };
        verify_commit_certificate(
            &chain.genesis.network_name,
            native_height,
            0,
            &native.id,
            &native.header,
            &native.commit,
            &set,
        )
        .unwrap();
        assert_eq!(
            native.header.data_hash,
            hash_transaction_data(&actual.block.transactions).unwrap()
        );
        assert_eq!(
            native.header.app_hash.as_slice(),
            if height == 1 {
                reference.target.content_digest.0.as_slice()
            } else {
                reference
                    .target
                    .application
                    .as_ref()
                    .unwrap()
                    .0
                    .0
                    .as_slice()
            }
        );
        assert_eq!(actual.parent.as_ref().unwrap(), &reference.target);
        let proposer = chain
            .genesis
            .validators
            .iter()
            .find(|validator| {
                validator_address(&validator.classical_public_key).as_slice()
                    == native.header.proposer_address
            })
            .unwrap()
            .owner;
        let previous_consensus_hash = native
            .header
            .last_block_id
            .as_ref()
            .map_or(B256::ZERO, |id| B256::from_slice(&id.hash));
        let input = ExecutionBlockInput {
            timestamp: u64::try_from(native.header.time.unwrap().seconds).unwrap(),
            proposer,
            previous_consensus_hash,
        };
        let prepared = execute_state_block(
            &reference,
            &input,
            &actual.block.transactions,
            &budget,
            CLONE_BYTES,
        )
        .unwrap();
        assert_eq!(prepared.commit, *actual);
        validate_state_commit(actual, &budget).unwrap();
        reference = prepared.commit;
    }
    assert_eq!(
        chain.commits[1].block.transactions,
        vec![signed_transaction()]
    );
    assert_eq!(chain.commits[1].block.header.gas_used, 43_106);
    assert_eq!(chain.commits[1].block.receipts.len(), 1);
    assert_eq!(chain.commits[1].state.accounts[&sender()].nonce, 1);
    assert_eq!(
        chain.commits[1].state.accounts[&Address::with_last_byte(0x42)].storage[&U256::ZERO],
        U256::from(99)
    );
    for commit in &chain.commits[2..] {
        assert!(commit.block.transactions.is_empty());
        assert!(commit.block.receipts.is_empty());
        assert_eq!(
            commit.state.accounts[&sender()],
            chain.commits[1].state.accounts[&sender()]
        );
    }
}
