// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{ImportExecutionWireStats, ImportWireError};
use super::scan_import_execution_stats_with_limit::scan_import_execution_stats_with_limit;
use crate::recovery::bounds::types::MAXIMUM_RECOVERY_BYTES;
use eve_state::StateBudget;

/// Preserve compact admission while sharing canonical counts and grammar.
pub(in crate::recovery::import::wire) fn scan_import_execution_stats(
    bytes: &[u8],
    budget: &StateBudget,
) -> Result<ImportExecutionWireStats, ImportWireError> {
    scan_import_execution_stats_with_limit(bytes, budget, MAXIMUM_RECOVERY_BYTES)
}
