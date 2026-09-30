use crate::support;

use alloy_primitives::{Address, B256, Bytes, U256, keccak256};
use eve_evm::{estimate_clone_reservation, execute_complete_state};
use eve_state::{
    JournalOperation, StateError, StateJournal, apply_state_journal, compute_evm_root,
    development_state_budget,
};
use support::{
    commits::structural_genesis,
    execution::{environment, executed_commit, execution_fixture},
    fixtures::{expected_root, fixture},
};

#[test]
fn ts04_journal_replay_matches_real_execution_without_losing_untouched_storage() {
    let parent = structural_genesis("execution_parent");
    let input = execution_fixture();
    let raw: Bytes = input["raw_transaction"].as_str().unwrap().parse().unwrap();
    let budget = development_state_budget();
    let outcome = execute_complete_state(
        &parent.state,
        &parent.target,
        &environment(&parent),
        &[raw],
        &budget,
        estimate_clone_reservation(&parent.state).unwrap(),
    )
    .unwrap();
    let replayed =
        apply_state_journal(&parent.state, &parent.target, &outcome.journal, &budget).unwrap();
    assert_eq!(replayed, outcome.state);
    let contract: Address = input["contract"].as_str().unwrap().parse().unwrap();
    assert_eq!(
        replayed.accounts[&contract].storage[&U256::from(1)],
        U256::from(11)
    );
    assert_eq!(
        compute_evm_root(&replayed.accounts).0,
        expected_root(&fixture("execution_post"), "evm_root")
    );
}

#[test]
fn ts01_zero_delete_recreate_and_code_release_follow_explicit_journal_order() {
    let parent = structural_genesis("two_slot_contract");
    let address = Address::repeat_byte(0x11);
    let hash = parent.state.accounts[&address].code_hash;
    let mut journal = StateJournal {
        parent: parent.target.clone(),
        target_height: 1,
        operations: vec![JournalOperation::PutStorage {
            address,
            slot: U256::ZERO,
            value: U256::ZERO,
        }],
    };
    let deleted = apply_state_journal(
        &parent.state,
        &parent.target,
        &journal,
        &development_state_budget(),
    )
    .unwrap();
    assert_eq!(
        compute_evm_root(&deleted.accounts).0,
        expected_root(&fixture("zero_slot_is_deleted"), "evm_root")
    );
    journal.operations = vec![
        JournalOperation::DeleteAccount { address },
        JournalOperation::PutAccount {
            address,
            nonce: 0,
            balance: U256::from(5),
            code_hash: keccak256([]),
        },
        JournalOperation::DeleteCode { code_hash: hash },
    ];
    let recreated = apply_state_journal(
        &parent.state,
        &parent.target,
        &journal,
        &development_state_budget(),
    )
    .unwrap();
    assert!(recreated.accounts[&address].storage.is_empty());
    assert!(!recreated.codes.contains_key(&hash));
    assert_eq!(
        compute_evm_root(&recreated.accounts).0,
        expected_root(&fixture("recreated_account"), "evm_root")
    );
    assert_eq!(parent.state.accounts[&address].storage.len(), 2);
}

#[test]
fn ts04_metadata_writes_preserve_slots_and_failed_later_operations_publish_nothing() {
    let parent = structural_genesis("two_slot_contract");
    let address = Address::repeat_byte(0x11);
    let mut journal = StateJournal {
        parent: parent.target.clone(),
        target_height: 1,
        operations: vec![JournalOperation::PutAccount {
            address,
            nonce: 2,
            balance: U256::from(5),
            code_hash: parent.state.accounts[&address].code_hash,
        }],
    };
    let result = apply_state_journal(
        &parent.state,
        &parent.target,
        &journal,
        &development_state_budget(),
    )
    .unwrap();
    assert_eq!(
        result.accounts[&address].storage,
        parent.state.accounts[&address].storage
    );
    journal.operations.push(JournalOperation::PutCode {
        code_hash: B256::ZERO,
        code: Bytes::from_static(&[0]),
    });
    assert!(
        apply_state_journal(
            &parent.state,
            &parent.target,
            &journal,
            &development_state_budget()
        )
        .is_err()
    );
    assert_eq!(parent.state.accounts[&address].nonce, 1);
    assert_eq!(parent.state.accounts[&address].balance, U256::from(100));
}

#[test]
fn ts04_stale_journal_parent_byte_caps_and_past_hash_rewrites_fail_closed() {
    let parent = structural_genesis("two_slot_contract");
    let mut journal = StateJournal {
        parent: parent.target.clone(),
        target_height: 1,
        operations: vec![JournalOperation::ClearStorage {
            address: Address::repeat_byte(0x11),
        }],
    };
    let mut budget = development_state_budget();
    budget.maximum_journal_bytes = 1;
    assert_eq!(
        apply_state_journal(&parent.state, &parent.target, &journal, &budget),
        Err(StateError::BudgetExceeded)
    );
    journal.parent.content_digest = B256::repeat_byte(0x77);
    assert_eq!(
        apply_state_journal(
            &parent.state,
            &parent.target,
            &journal,
            &development_state_budget()
        ),
        Err(StateError::ParentMismatch)
    );
    let (_, current) = executed_commit();
    let rewrite = StateJournal {
        parent: current.target.clone(),
        target_height: 2,
        operations: vec![JournalOperation::SetExecutionBlockHash {
            height: 0,
            hash: eve_state::ExecutionBlockHash(B256::repeat_byte(0x88)),
        }],
    };
    assert_eq!(
        apply_state_journal(
            &current.state,
            &current.target,
            &rewrite,
            &development_state_budget()
        ),
        Err(StateError::VersionMismatch)
    );
}
