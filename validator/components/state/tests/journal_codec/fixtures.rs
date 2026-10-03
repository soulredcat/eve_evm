// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{
    Address, B256, Bytes, ExecutionBlockHash, JournalOperation, StateJournal, SystemNamespace,
    SystemRecord, SystemValue, U256, encode_state_version, hash_system_key,
};

pub use super::system_records::system_records;

pub fn list(fields: &[Vec<u8>]) -> Vec<u8> {
    let payload_length = fields.iter().map(Vec::len).sum();
    let mut bytes = Vec::new();
    alloy_rlp::Header {
        list: true,
        payload_length,
    }
    .encode(&mut bytes);
    for field in fields {
        bytes.extend_from_slice(field);
    }
    bytes
}

pub fn raw_journal(operations: &[Vec<u8>]) -> Vec<u8> {
    list(&[
        alloy_rlp::encode(b"EVE_STATE_JOURNAL_V1".as_slice()),
        encode_state_version(&super::support::genesis().target)
            .unwrap()
            .to_vec(),
        alloy_rlp::encode(1_u64),
        list(operations),
    ])
}

pub fn journal(operations: Vec<JournalOperation>) -> StateJournal {
    StateJournal {
        parent: super::support::genesis().target,
        target_height: 1,
        operations,
    }
}

pub fn parameter_record() -> SystemRecord {
    SystemRecord {
        schema_version: 1,
        namespace: SystemNamespace::Parameter,
        logical_key: Bytes::from_static(b"gas"),
        value: SystemValue::Parameter {
            name: Bytes::from_static(b"gas"),
            value: Bytes::from_static(b"value"),
        },
    }
}

pub fn put_system(record: SystemRecord) -> JournalOperation {
    JournalOperation::PutSystem {
        key: hash_system_key(record.namespace, &record.logical_key).unwrap(),
        record,
    }
}

pub fn every_operation() -> StateJournal {
    let address = Address::repeat_byte(5);
    let hash = B256::repeat_byte(6);
    journal(vec![
        JournalOperation::PutAccount {
            address,
            nonce: u64::MAX,
            balance: U256::MAX,
            code_hash: hash,
        },
        JournalOperation::DeleteAccount { address },
        JournalOperation::PutStorage {
            address,
            slot: U256::MAX,
            value: U256::ZERO,
        },
        JournalOperation::DeleteStorage {
            address,
            slot: U256::MAX,
        },
        JournalOperation::ClearStorage { address },
        JournalOperation::PutCode {
            code_hash: hash,
            code: Bytes::from_static(&[0, 0x7f, 0x80, 0xff]),
        },
        JournalOperation::DeleteCode { code_hash: hash },
        put_system(parameter_record()),
        JournalOperation::DeleteSystem { key: hash },
        JournalOperation::SetExecutionBlockHash {
            height: u64::MAX,
            hash: ExecutionBlockHash(hash),
        },
    ])
}
