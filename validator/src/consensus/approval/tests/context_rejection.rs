// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::proposal_fixture;
use crate::consensus::{
    approval::{ApprovalError, create_execution_approval},
    signing::{
        open_durable_signer,
        tests::{temporary_fixture, test_key},
    },
    transport::proposals::{EngineProposalBinding, fixture_verified_local_engine_proposal},
};
use eve_consensus_comet::consensus::authentication::ConsensusAuthenticationRequirement;
use eve_state::{Address, development_state_budget};
use eve_storage::records::development_opaque_record_budget;

#[test]
fn wrong_genesis_chain_epoch_profile_and_timestamp_never_create_approval() {
    let (_directory, fixture) = temporary_fixture();
    let signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    for index in 0..5 {
        let mut request = proposal_fixture(&fixture.config, Vec::new())
            .request()
            .clone();
        let mut binding = EngineProposalBinding {
            genesis_hash: fixture.config.genesis_hash,
            chain_id: fixture.config.chain_id.clone(),
            authentication: fixture.config.authentication,
            key_epoch: fixture.config.key_epoch,
            proposer_owner: Address::repeat_byte(1),
            previous_consensus_hash: [0; 32],
        };
        match index {
            0 => binding.genesis_hash[0] ^= 1,
            1 => binding.chain_id = "wrong-chain".into(),
            2 => binding.key_epoch += 1,
            3 => binding.authentication = ConsensusAuthenticationRequirement::ClassicalAndMldsa65,
            _ => request.time.as_mut().unwrap().nanos = 1_000_000_000,
        }
        let token = fixture_verified_local_engine_proposal(request, binding);
        assert!(
            create_execution_approval(
                &signer,
                &token,
                &development_state_budget(),
                256 * 1024 * 1024
            )
            .is_err()
        );
    }
}

#[test]
fn malformed_transaction_and_missing_clone_reservation_fail_actual_execution() {
    let (_directory, fixture) = temporary_fixture();
    let signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    assert!(matches!(
        create_execution_approval(
            &signer,
            &proposal_fixture(&fixture.config, vec![vec![0]]),
            &development_state_budget(),
            256 * 1024 * 1024
        ),
        Err(ApprovalError::InvalidExecution(
            eve_evm::CompleteExecutionError::Execution(_)
        ))
    ));
    let missing = create_execution_approval(
        &signer,
        &proposal_fixture(&fixture.config, Vec::new()),
        &development_state_budget(),
        0,
    );
    match missing {
        Err(ApprovalError::Unavailable {
            reason,
            cause: Some(eve_evm::CompleteExecutionError::CloneReservation { .. }),
        }) => assert!(!reason.is_empty()),
        _ => panic!("clone reservation failure must be unavailable, not execution-invalid"),
    }
}
