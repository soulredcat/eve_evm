// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    BlockPayload, Bytes, JournalDecodePreflight, StateBudget, StateError, StateJournal,
    StateVersion,
};

pub(super) const REQUEST_DOMAIN: &[u8] = b"EVE_DELTA_REQ_V1";
pub(super) const PAYLOAD_DOMAIN: &[u8] = b"EVE_STATE_DELTA_V1";
pub(super) const CHUNK_DOMAIN: &[u8] = b"EVE_DELTA_CHUNK_V1";
pub(super) const MAXIMUM_EXECUTION_BYTES: usize = 8_404_140;
pub const MAXIMUM_STATE_DELTA_CHUNK_BYTES: usize = 4_194_304;
pub const MAXIMUM_STATE_DELTA_REQUEST_BYTES: usize = 4_136;
/// 18-byte domain, two u32 lengths, 8 MiB journal and full bounded execution.
pub const MAXIMUM_STATE_DELTA_PAYLOAD_BYTES: usize = 16_792_774;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateDeltaError {
    State(StateError),
    UnsupportedVersion,
    MalformedEncoding,
    NonCanonicalEncoding,
    BudgetExceeded,
    AllocationFailed,
    ChecksumMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateDeltaPayload {
    pub journal: StateJournal,
    pub execution: BlockPayload,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateDeltaRequest {
    pub parent: StateVersion,
    pub target_height: u64,
    pub offset: u64,
    pub maximum_chunk_bytes: u32,
}

/// Source metadata and SHA integrity only; no certificate or execution authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateDeltaChunk {
    pub parent: StateVersion,
    pub target: StateVersion,
    pub durable_tip: StateVersion,
    pub body_sha256: [u8; 32],
    pub total_length: u64,
    pub offset: u64,
    pub data: Bytes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateDeltaExecutionStats {
    pub encoded_bytes: usize,
    pub header_bytes: usize,
    pub transaction_count: usize,
    pub transaction_bytes: usize,
    pub receipt_count: usize,
    pub receipt_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateDeltaPayloadStats {
    pub encoded_bytes: usize,
    pub journal: JournalDecodePreflight,
    pub execution: StateDeltaExecutionStats,
}

/// Sealed same-byte/frozen-budget admission facts. It establishes no finality.
#[derive(Debug)]
pub struct StateDeltaPayloadPreflight<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) budget: StateBudget,
    pub(super) journal: &'a [u8],
    pub(super) execution: &'a [u8],
    pub(super) stats: StateDeltaPayloadStats,
}
