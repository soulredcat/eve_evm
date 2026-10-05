// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Conservative peak additions. Deletes never reduce these ordered write counts.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct JournalCandidateGrowth {
    pub operation_count: usize,
    pub accounts: usize,
    pub slots: usize,
    pub codes: usize,
    pub code_bytes: usize,
    pub system: usize,
    pub history: usize,
    pub journal_bytes: usize,
}
