// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::StateStorageBudget;
use alloy_primitives::B256;
use eve_state::{BlockPayload, StateVersion};
use rocksdb::{DB, SnapshotWithThreadMode};
use std::sync::{Arc, atomic::AtomicUsize};
#[derive(Clone, Copy, Debug)]
pub struct HistoryReadBudget {
    /// Sum of projection row value bytes, including parent/version/root/commit identity metadata.
    pub maximum_block_bytes: usize,
    pub maximum_rebuild_blocks: usize,
    pub maximum_index_batch_bytes: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionLocation {
    pub height: u64,
    pub transaction_index: u32,
}
#[derive(Debug, Clone)]
pub struct HistoryIndexStatus {
    pub indexed_height: Option<u64>,
    pub target_height: u64,
    pub complete: bool,
}
#[derive(Debug, Clone)]
pub struct RetainedBlockProjection {
    pub version: StateVersion,
    pub commit_identity: B256,
    pub block: BlockPayload,
}
/// One bounded successful-sync snapshot; no finality authority.
pub struct HistorySnapshot<'a> {
    pub(crate) snapshot: SnapshotWithThreadMode<'a, DB>,
    pub(crate) head: StateVersion,
    pub(crate) database_sequence: u64,
    pub(crate) storage_budget: StateStorageBudget,
    pub(crate) read_budget: HistoryReadBudget,
    pub(crate) leases: Arc<AtomicUsize>,
}
