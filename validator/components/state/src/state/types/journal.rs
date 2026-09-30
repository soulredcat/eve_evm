use alloy_primitives::{Address, B256, Bytes, U256};
use eve_protocol_config::records::{ExecutionBlockHash, SystemRecord};

use super::StateVersion;

/// Explicit execution order: delete then recreate is different from metadata update.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JournalOperation {
    PutAccount {
        address: Address,
        nonce: u64,
        balance: U256,
        code_hash: B256,
    },
    DeleteAccount {
        address: Address,
    },
    PutStorage {
        address: Address,
        slot: U256,
        value: U256,
    },
    DeleteStorage {
        address: Address,
        slot: U256,
    },
    ClearStorage {
        address: Address,
    },
    PutCode {
        code_hash: B256,
        code: Bytes,
    },
    DeleteCode {
        code_hash: B256,
    },
    PutSystem {
        key: B256,
        record: SystemRecord,
    },
    DeleteSystem {
        key: B256,
    },
    SetExecutionBlockHash {
        height: u64,
        hash: ExecutionBlockHash,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateJournal {
    pub parent: StateVersion,
    pub target_height: u64,
    pub operations: Vec<JournalOperation>,
}
