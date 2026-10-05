// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use crate::recovery_support::{self as support, CLONE_BYTES};
use eve_finality_verifier::{
    RecoveryError, initialize_development_recovery, prepare_development_recovery,
    recovery_state_commit,
};
use eve_state::{
    Address, B256, SystemValue, U256, build_state_commit, compute_evm_root,
    development_state_budget,
};

#[test]
fn auxiliary_target_content_digest_must_be_reconstructed_by_execution() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    let mut record = support::envelope(&chain, 1);
    record.expected.content_digest = B256::repeat_byte(0xba);
    assert_eq!(
        prepare_development_recovery(
            &parent,
            Arc::new(record),
            &development_state_budget(),
            CLONE_BYTES,
        )
        .unwrap_err(),
        RecoveryError::ReplayMismatch,
    );
    assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn execution_context_is_derived_from_certified_native_data() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    for field in 0..3 {
        let mut record = support::envelope(&chain, 1);
        match field {
            0 => {
                record.expected.timestamp += 1;
                record.execution.header.timestamp += 1;
            }
            1 => record.execution.header.beneficiary = Address::repeat_byte(0xaf),
            2 => record.execution.header.mix_hash = B256::repeat_byte(0xb0),
            _ => unreachable!(),
        }
        record.expected = build_state_commit(
            Some(record.parent.clone()),
            chain.commits[1].state.clone(),
            record.execution.clone(),
            &development_state_budget(),
        )
        .unwrap()
        .target;
        record.lookahead.frame.header.app_hash = record.expected.application.unwrap().0.0.to_vec();
        support::resign(&mut record.lookahead.frame, &chain.genesis);
        assert_eq!(
            prepare_development_recovery(
                &parent,
                Arc::new(record),
                &development_state_budget(),
                CLONE_BYTES,
            )
            .unwrap_err(),
            RecoveryError::ReplayMismatch,
        );
    }
    assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn insufficient_execution_clone_parameter_rejects_without_publishing_or_advancing_history() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    assert!(matches!(
        prepare_development_recovery(
            &parent,
            Arc::new(support::envelope(&chain, 1)),
            &development_state_budget(),
            0,
        ),
        Err(RecoveryError::Execution(
            eve_evm::CompleteExecutionError::CloneReservation { .. }
        )),
    ));
    assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
    assert!(
        prepare_development_recovery(
            &parent,
            Arc::new(support::envelope(&chain, 1)),
            &development_state_budget(),
            CLONE_BYTES,
        )
        .is_ok()
    );
}

#[test]
fn certified_peer_roots_cannot_replace_reconstructed_evm_and_system_outcomes() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_development_recovery(&chain.genesis, &budget).unwrap();
    for field in 0..2 {
        let mut record = support::envelope(&chain, 1);
        let mut false_state = chain.commits[1].state.clone();
        if field == 0 {
            false_state
                .accounts
                .get_mut(&Address::with_last_byte(0x42))
                .unwrap()
                .storage
                .insert(U256::ZERO, U256::from(100));
            record.execution.header.state_root = compute_evm_root(&false_state.accounts).0;
        } else {
            let fee = false_state
                .system
                .values_mut()
                .find_map(|record| {
                    if let SystemValue::Fee { burned, .. } = &mut record.value {
                        Some(burned)
                    } else {
                        None
                    }
                })
                .unwrap();
            *fee += U256::from(1);
        }
        record.expected = build_state_commit(
            Some(record.parent.clone()),
            false_state,
            record.execution.clone(),
            &budget,
        )
        .unwrap()
        .target;
        record.lookahead.frame.header.app_hash = record.expected.application.unwrap().0.0.to_vec();
        support::resign(&mut record.lookahead.frame, &chain.genesis);
        assert_eq!(
            prepare_development_recovery(&parent, Arc::new(record), &budget, CLONE_BYTES)
                .unwrap_err(),
            RecoveryError::ReplayMismatch,
        );
    }
    assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
}
