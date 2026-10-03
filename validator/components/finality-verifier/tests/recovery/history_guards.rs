// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use crate::recovery_support::{self as support, CLONE_BYTES};
use eve_finality_verifier::{
    RecoveryError, initialize_development_recovery, into_recovery_state,
    prepare_development_recovery, recovery_state_commit,
};
use eve_state::{Bytes, development_state_budget};

#[test]
fn invalid_certificate_wrong_parent_and_altered_lookahead_data_leave_parent_unchanged() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    for field in 0..9 {
        let mut record = support::envelope(&chain, 1);
        match field {
            0 => record.finalized.commit.signatures[0].signature[0] ^= 1,
            1 => record.lookahead.frame.commit.signatures[0].signature[0] ^= 1,
            2 => {
                record
                    .lookahead
                    .frame
                    .header
                    .last_block_id
                    .as_mut()
                    .unwrap()
                    .part_set_header
                    .as_mut()
                    .unwrap()
                    .hash[0] ^= 1;
                support::resign(&mut record.lookahead.frame, &chain.genesis);
            }
            3 => record
                .lookahead
                .transactions
                .push(Bytes::from_static(b"unbound-data")),
            4 => {
                for signature in &mut record.finalized.commit.signatures[2..] {
                    signature.block_id_flag = 1;
                    signature.validator_address.clear();
                    signature.timestamp = None;
                    signature.signature.clear();
                }
            }
            5 => {
                record.finalized.header.app_hash[0] ^= 1;
                support::resign(&mut record.finalized, &chain.genesis);
            }
            6 => {
                record.lookahead.frame.header.app_hash[0] ^= 1;
                support::resign(&mut record.lookahead.frame, &chain.genesis);
            }
            7 => {
                record.finalized.header.chain_id.push_str("-foreign");
                support::resign(&mut record.finalized, &chain.genesis);
            }
            8 => {
                record.lookahead.frame.header.version.as_mut().unwrap().app += 1;
                support::resign(&mut record.lookahead.frame, &chain.genesis);
            }
            _ => unreachable!(),
        }
        assert!(
            prepare_development_recovery(
                &parent,
                Arc::new(record),
                &development_state_budget(),
                CLONE_BYTES,
            )
            .is_err()
        );
        assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
    }
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
fn a_second_record_cannot_replace_an_already_certified_lookahead_proof() {
    let chain = support::recovery_chain();
    let initial =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    let first = prepare_development_recovery(
        &initial,
        Arc::new(support::envelope(&chain, 1)),
        &development_state_budget(),
        CLONE_BYTES,
    )
    .unwrap();
    let parent = into_recovery_state(first);
    let mut record = support::envelope(&chain, 2);
    record.finalized.header.last_results_hash = vec![0x75; 32];
    support::resign(&mut record.finalized, &chain.genesis);
    record.lookahead.frame.header.last_block_id = Some(record.finalized.block_id.clone());
    support::resign(&mut record.lookahead.frame, &chain.genesis);
    assert_eq!(
        prepare_development_recovery(
            &parent,
            Arc::new(record),
            &development_state_budget(),
            CLONE_BYTES,
        )
        .unwrap_err(),
        RecoveryError::WrongLookahead,
    );
    assert_eq!(recovery_state_commit(&parent), &chain.commits[1]);
    assert!(
        prepare_development_recovery(
            &parent,
            Arc::new(support::envelope(&chain, 2)),
            &development_state_budget(),
            CLONE_BYTES,
        )
        .is_ok()
    );
}

#[test]
fn native_set_hashes_and_proposer_addresses_cannot_create_new_owner_authority() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    for field in 0..4 {
        let mut record = support::envelope(&chain, 1);
        let frame = if field < 2 {
            &mut record.finalized
        } else {
            &mut record.lookahead.frame
        };
        if field % 2 == 0 {
            frame.header.next_validators_hash = vec![0xa5; 32];
        } else {
            frame.header.proposer_address = vec![0xa6; 20];
        }
        support::resign(frame, &chain.genesis);
        let error = prepare_development_recovery(
            &parent,
            Arc::new(record),
            &development_state_budget(),
            CLONE_BYTES,
        )
        .unwrap_err();
        assert_eq!(
            error,
            if field % 2 == 0 {
                RecoveryError::UnsupportedValidatorTransition
            } else {
                RecoveryError::UnknownProposer
            },
        );
    }
    assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
}
