// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(crate) mod indexing;
pub(crate) mod keys;
mod reading;
#[cfg(test)]
#[path = "../../../tests/history_failure/mod.rs"]
mod tests;
mod types;
pub(crate) mod validation;
pub use indexing::ensure_history_index;
pub use reading::{
    capture_history_snapshot, lookup_execution_hash, lookup_transaction, read_history_block,
};
pub use types::{
    HistoryIndexStatus, HistoryReadBudget, HistorySnapshot, RetainedBlockProjection,
    TransactionLocation,
};
