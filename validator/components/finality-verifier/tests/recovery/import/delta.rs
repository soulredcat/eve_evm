// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_finality_verifier::{
    ImportError, RecoveryError, imported_state_commit, imported_transition_state,
    initialize_authenticated_import, prepare_authenticated_import,
};
use eve_state::{Address, B256, Bytes, JournalOperation, U256, development_state_budget};

use super::support::input;
use crate::recovery_support::{self as support, CLONE_BYTES};

#[test]
fn changed_accounts_storage_system_ledger_and_code_cannot_match_certified_roots() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    for field in 0..5 {
        let mut delta = input(&chain, 1);
        match field {
            0 => {
                let account = delta
                    .journal
                    .operations
                    .iter_mut()
                    .find(|operation| {
                        matches!(operation, JournalOperation::PutAccount { address, .. }
                        if *address == support::sender())
                    })
                    .unwrap();
                if let JournalOperation::PutAccount { nonce, .. } = account {
                    *nonce += 1;
                }
            }
            1 => {
                let storage = delta
                    .journal
                    .operations
                    .iter_mut()
                    .find(|operation| matches!(operation, JournalOperation::PutStorage { .. }))
                    .unwrap();
                if let JournalOperation::PutStorage { value, .. } = storage {
                    *value = U256::from(100);
                }
            }
            2 => {
                let index = delta
                    .journal
                    .operations
                    .iter()
                    .position(|operation| matches!(operation, JournalOperation::PutSystem { .. }))
                    .unwrap();
                delta.journal.operations.remove(index);
            }
            3 => delta.journal.operations.insert(
                0,
                JournalOperation::PutCode {
                    code_hash: B256::repeat_byte(0xa1),
                    code: Bytes::from_static(&[0x00]),
                },
            ),
            4 => delta.journal.operations.insert(
                0,
                JournalOperation::DeleteCode {
                    code_hash: *chain.commits[0].state.codes.keys().next().unwrap(),
                },
            ),
            _ => unreachable!(),
        }
        assert!(
            prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES).is_err()
        );
        assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
    }
    assert!(
        prepare_authenticated_import(&parent, Arc::new(input(&chain, 1)), &budget, CLONE_BYTES)
            .is_ok()
    );
}

#[test]
fn malformed_or_changed_receipts_remain_untrusted_even_with_valid_native_history() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let mut delta = input(&chain, 1);
    delta.execution.receipts[0] = Bytes::from_static(&[0xc0]);
    assert!(prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES).is_err());
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn changed_fee_system_state_is_rejected_by_h_plus_one_even_when_evm_root_matches() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let mut delta = input(&chain, 1);
    let index = delta
        .journal
        .operations
        .iter()
        .position(|operation| matches!(operation, JournalOperation::PutSystem { .. }))
        .unwrap();
    delta.journal.operations.remove(index);
    assert!(matches!(
        prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES).unwrap_err(),
        ImportError::Recovery(RecoveryError::Finality(_)),
    ));
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn ordered_delete_recreate_and_storage_replacement_preserve_the_certified_final_outcome() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let mut delta = input(&chain, 1);
    let address = Address::with_last_byte(0x42);
    let account = &chain.commits[1].state.accounts[&address];
    let index = delta.journal.operations.iter().position(|operation| {
        matches!(operation, JournalOperation::PutStorage { address: target, .. } if *target == address)
    }).unwrap();
    delta.journal.operations.splice(
        index..=index,
        [
            JournalOperation::DeleteAccount { address },
            JournalOperation::PutAccount {
                address,
                nonce: account.nonce,
                balance: account.balance,
                code_hash: account.code_hash,
            },
            JournalOperation::PutStorage {
                address,
                slot: U256::ZERO,
                value: U256::from(99),
            },
        ],
    );
    let transition =
        prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES).unwrap();
    assert_eq!(
        imported_state_commit(imported_transition_state(&transition)),
        &chain.commits[1]
    );
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}
