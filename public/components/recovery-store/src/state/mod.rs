// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Complete local durable state. Consistency/durability never grants finality.
mod budget;
mod encoding;
mod history;
mod repository;
mod service;
mod snapshots;
mod types;

pub use budget::{
    StateStorageBudget, development_state_storage_budget, validate_state_storage_budget,
};
pub use history::{
    HistoryIndexStatus, HistoryReadBudget, HistorySnapshot, RetainedBlockProjection,
    TransactionLocation, capture_history_snapshot, ensure_history_index, lookup_execution_hash,
    lookup_transaction, read_history_block,
};
pub use repository::{commit_state, open_state_repository, state_reader};
pub use service::{
    StateService, create_state_service, read_cached_state_service, read_state_service,
};
pub use snapshots::{
    LocalSnapshotManifest, SnapshotCommitReference, activate_snapshot_namespace,
    capture_state_snapshot, export_state_snapshot, read_snapshot_commit,
    snapshot_commit_encoded_length,
};
pub use types::{
    CommitDisposition, DurableStateAck, ImmutableStateView, StateReader, StateRepository,
    StateSnapshot,
};

#[cfg(test)]
#[path = "../../tests/state_failure/mod.rs"]
mod tests;
