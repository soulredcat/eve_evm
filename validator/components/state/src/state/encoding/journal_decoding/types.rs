// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{Address, B256, ExecutionBlockHash, U256};

/// Local decode charges, not final-state cardinalities or authenticated finality.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JournalDecodePreflight {
    pub encoded_bytes: usize,
    pub operation_count: usize,
    /// Exact requested Vec element storage, excluding allocator metadata/alignment.
    pub operation_allocation_bytes: usize,
    pub code_operations: usize,
    pub code_bytes: usize,
    pub system_operations: usize,
    pub system_encoded_bytes: usize,
    /// Conservative retained/copied byte charge: all system RLP leaf payloads.
    pub system_payload_bytes: usize,
    /// Conservative byte-field count: all system RLP leaves, including primitives.
    pub system_leaf_count: usize,
    pub maximum_system_record_bytes: usize,
    /// Exact UTF-8 parent network payload; the maintained decoder copies it twice.
    pub parent_network_name_bytes: usize,
    /// Matches the existing encoder's conservative operation-byte policy.
    pub conservative_journal_bytes: usize,
    /// Conservative bounded version/system codec scratch estimate, not an RSS cap.
    pub conservative_codec_scratch_bytes: usize,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum BorrowedJournalOperation<'a> {
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
        code: &'a [u8],
    },
    DeleteCode {
        code_hash: B256,
    },
    PutSystem {
        key: B256,
        record: &'a [u8],
    },
    DeleteSystem {
        key: B256,
    },
    SetExecutionBlockHash {
        height: u64,
        hash: ExecutionBlockHash,
    },
}
