// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::StateBudget;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StateCommitDecodeStats {
    pub encoded_bytes: usize,
    pub state_encoded_bytes: usize,
    pub accounts: usize,
    pub storage_slots: usize,
    pub codes: usize,
    pub code_bytes: usize,
    pub maximum_code_blob_bytes: usize,
    pub system_records: usize,
    pub system_encoded_bytes: usize,
    pub system_payload_bytes: usize,
    pub system_leaf_count: usize,
    pub maximum_system_record_bytes: usize,
    pub history_entries: usize,
    pub parent_network_bytes: usize,
    pub target_network_bytes: usize,
    pub state_network_bytes: usize,
    pub header_encoded_bytes: usize,
    pub header_payload_bytes: usize,
    pub header_leaf_count: usize,
    pub transaction_count: usize,
    pub transaction_bytes: usize,
    pub receipt_count: usize,
    pub receipt_bytes: usize,
    /// Conservative Vec element envelope for maintained incremental push/growth.
    pub block_vector_allocation_bytes: usize,
    pub bounded_codec_scratch_bytes: usize,
}

/// Sealed immutable input and frozen budget. Detached counts cannot decode another buffer.
#[derive(Debug)]
pub struct StateCommitPreflight<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) target_bytes: &'a [u8],
    pub(super) budget: StateBudget,
    pub(super) stats: StateCommitDecodeStats,
}
