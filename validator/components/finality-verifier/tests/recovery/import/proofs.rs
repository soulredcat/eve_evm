// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_finality_verifier::{
    ImportError, RecoveryError, imported_state_commit, initialize_authenticated_import,
    into_imported_state, prepare_authenticated_import,
};
use eve_state::{Bytes, development_state_budget};

use super::support::input;
use crate::recovery_support::{self as support, CLONE_BYTES};

#[test]
fn altered_certificates_native_data_application_chain_or_profile_never_advance_import() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    for field in 0..8 {
        let mut delta = input(&chain, 1);
        match field {
            0 => delta.finalized.commit.signatures[0].signature[0] ^= 1,
            1 => delta.lookahead.frame.commit.signatures[0].signature[0] ^= 1,
            2 => delta
                .lookahead
                .transactions
                .push(Bytes::from_static(b"unbound-data")),
            3 => {
                let mut transaction = delta.execution.transactions[0].to_vec();
                *transaction.last_mut().unwrap() ^= 1;
                delta.execution.transactions[0] = transaction.into();
            }
            4 => {
                delta.lookahead.frame.header.app_hash[0] ^= 1;
                support::resign(&mut delta.lookahead.frame, &chain.genesis);
            }
            5 => {
                delta.finalized.header.chain_id.push_str("-foreign");
                support::resign(&mut delta.finalized, &chain.genesis);
            }
            6 => {
                delta.lookahead.frame.header.version.as_mut().unwrap().app += 1;
                support::resign(&mut delta.lookahead.frame, &chain.genesis);
            }
            7 => {
                for signature in &mut delta.finalized.commit.signatures[2..] {
                    signature.block_id_flag = 1;
                    signature.validator_address.clear();
                    signature.timestamp = None;
                    signature.signature.clear();
                }
            }
            _ => unreachable!(),
        }
        assert!(
            prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES).is_err()
        );
        assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
    }
}

#[test]
fn fixed_enrolled_sets_and_proposer_owners_cannot_be_replaced_by_signed_metadata() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    for field in 0..4 {
        let mut delta = input(&chain, 1);
        let frame = if field < 2 {
            &mut delta.finalized
        } else {
            &mut delta.lookahead.frame
        };
        if field % 2 == 0 {
            frame.header.next_validators_hash = vec![0xb1; 32];
        } else {
            frame.header.proposer_address = vec![0xb2; 20];
        }
        support::resign(frame, &chain.genesis);
        assert_eq!(
            prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES)
                .unwrap_err(),
            ImportError::Recovery(if field % 2 == 0 {
                RecoveryError::UnsupportedValidatorTransition
            } else {
                RecoveryError::UnknownProposer
            }),
        );
    }
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn subsequent_import_cannot_replace_already_certified_lookahead() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let first = into_imported_state(
        prepare_authenticated_import(&parent, Arc::new(input(&chain, 1)), &budget, CLONE_BYTES)
            .unwrap(),
    );
    let mut delta = input(&chain, 2);
    delta.finalized.header.consensus_hash[0] ^= 1;
    support::resign(&mut delta.finalized, &chain.genesis);
    assert_eq!(
        prepare_authenticated_import(&first, Arc::new(delta), &budget, CLONE_BYTES).unwrap_err(),
        ImportError::Recovery(RecoveryError::WrongLookahead),
    );
    assert_eq!(imported_state_commit(&first), &chain.commits[1]);
}
