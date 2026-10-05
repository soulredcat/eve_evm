// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use crate::{
    native,
    recovery_support::{self as support, CLONE_BYTES},
};
use eve_evm::{ExecutionBlockInput, execute_state_block};
use eve_finality_verifier::{
    CompactRecoveryEnvelopeV1, FinalityError, NativeDataFrame, RecoveryError,
    initialize_development_recovery, prepare_development_recovery, recovery_state_commit,
};
use eve_protocol_config::genesis::ScheduledUpgrade;
use eve_state::{
    B256, Bytes, SecurityProfile, development_state_budget, initialize_development_state,
};

#[test]
fn unsupported_profile_and_invalid_local_genesis_cannot_create_replay_authority() {
    let mut genesis = support::funded_genesis();
    genesis.profile = SecurityProfile::HybridExperimental;
    assert!(initialize_development_recovery(&genesis, &development_state_budget()).is_err());
    genesis.profile = SecurityProfile::ClassicalDev;
    genesis.validators[0].classical_public_key = [0; 32];
    assert!(initialize_development_recovery(&genesis, &development_state_budget()).is_err());
}

#[test]
fn scheduled_upgrade_in_lookahead_fails_closed_before_current_state_is_applied() {
    let mut genesis = support::funded_genesis();
    genesis.upgrades.push(ScheduledUpgrade {
        activation_height: 2,
        protocol_version: 2,
        profile: SecurityProfile::ClassicalDev,
        code_digest: B256::repeat_byte(0x71),
        migration_id: Bytes::from_static(b"unsupported-recovery-test-upgrade"),
    });
    let budget = development_state_budget();
    let initial = initialize_development_state(&genesis, &budget).unwrap();
    let first = native::frame(&genesis, 1, None, initial.target.content_digest.0);
    let owner = genesis
        .validators
        .iter()
        .find(|validator| {
            eve_consensus_comet::consensus::certificates::validator_address(
                &validator.classical_public_key,
            )
            .as_slice()
                == first.header.proposer_address
        })
        .unwrap()
        .owner;
    let prepared = execute_state_block(
        &initial,
        &ExecutionBlockInput {
            timestamp: u64::try_from(first.header.time.unwrap().seconds).unwrap(),
            proposer: owner,
            previous_consensus_hash: B256::ZERO,
        },
        &[],
        &budget,
        CLONE_BYTES,
    )
    .unwrap();
    let next = native::frame(
        &genesis,
        2,
        Some(first.id.clone()),
        prepared.commit.target.application.unwrap().0.0,
    );
    let record = CompactRecoveryEnvelopeV1 {
        parent: initial.target.clone(),
        expected: prepared.commit.target,
        execution: prepared.commit.block,
        finalized: support::native_frame(&first),
        lookahead: NativeDataFrame {
            frame: support::native_frame(&next),
            transactions: Vec::new(),
        },
    };
    let parent = initialize_development_recovery(&genesis, &budget).unwrap();
    assert_eq!(
        prepare_development_recovery(&parent, Arc::new(record), &budget, CLONE_BYTES).unwrap_err(),
        RecoveryError::Finality(FinalityError::UnsupportedActivation),
    );
    assert_eq!(recovery_state_commit(&parent), &initial);
}
