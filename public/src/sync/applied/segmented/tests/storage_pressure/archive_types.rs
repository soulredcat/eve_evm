// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_storage::{
    records::{OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordIdentity},
    state::{HistoryReadBudget, StateStorageBudget},
};

pub(super) struct ArchiveMaintenance {
    pub _directory: tempfile::TempDir,
    pub opaque_path: std::path::PathBuf,
    pub index_path: std::path::PathBuf,
    pub opaque_budget: OpaqueRecordBudget,
    pub state_budget: StateStorageBudget,
    pub history_budget: HistoryReadBudget,
    pub identity: OpaqueRecordIdentity,
    pub head: OpaqueRecordCursor,
}
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct ArchiveJobCounts {
    pub compactions: usize,
    pub index_lookups: usize,
}
