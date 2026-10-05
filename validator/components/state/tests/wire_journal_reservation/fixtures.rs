// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{
    B256, Bytes, ExecutionBlockHash, JournalOperation, StateCommit, StateJournal, U256,
    hash_system_key,
};

pub fn journal(parent: &StateCommit, operations: Vec<JournalOperation>) -> StateJournal {
    StateJournal {
        parent: parent.target.clone(),
        target_height: parent.target.height + 1,
        operations,
    }
}

pub fn every_operation(parent: &StateCommit) -> Vec<JournalOperation> {
    let address = super::support::contract();
    let account = &parent.state.accounts[&address];
    let record = parent.state.system.values().next().unwrap().clone();
    let key = hash_system_key(record.namespace, &record.logical_key).unwrap();
    vec![
        JournalOperation::PutAccount {
            address,
            nonce: 0,
            balance: account.balance,
            code_hash: account.code_hash,
        },
        JournalOperation::DeleteAccount { address },
        JournalOperation::PutStorage {
            address,
            slot: U256::ZERO,
            value: U256::ZERO,
        },
        JournalOperation::DeleteStorage {
            address,
            slot: U256::ZERO,
        },
        JournalOperation::ClearStorage { address },
        JournalOperation::PutCode {
            code_hash: B256::ZERO,
            code: Bytes::from_static(&[1, 2, 3]),
        },
        JournalOperation::DeleteCode {
            code_hash: B256::ZERO,
        },
        JournalOperation::PutSystem { key, record },
        JournalOperation::DeleteSystem { key },
        JournalOperation::SetExecutionBlockHash {
            height: 2,
            hash: ExecutionBlockHash(B256::ZERO),
        },
    ]
}
