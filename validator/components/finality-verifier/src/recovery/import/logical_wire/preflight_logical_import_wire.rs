// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::wire::{
    ImportWireError, ImportWireStats, scan_import_execution_stats_with_limit,
    scan_import_lookahead_stats, scan_import_native_stats,
};
use super::{
    LogicalImportWirePreflight, slice_logical_import_wire::slice_logical_import_wire,
    types::MAXIMUM_LOGICAL_EXECUTION_BYTES,
};
use eve_state::{StateBudget, preflight_state_journal};

/// Full borrowed component/count admission before decoded allocation. Same raw
/// input and frozen budget remain attached; no certificate or outcome is verified.
pub fn preflight_logical_import_wire<'a>(
    bytes: &'a [u8],
    budget: &StateBudget,
) -> Result<LogicalImportWirePreflight<'a>, ImportWireError> {
    let slices = slice_logical_import_wire(bytes, budget)?;
    let stats = ImportWireStats {
        encoded_bytes: bytes.len(),
        journal: preflight_state_journal(slices.journal, budget).map_err(ImportWireError::State)?,
        execution: scan_import_execution_stats_with_limit(
            slices.execution,
            budget,
            MAXIMUM_LOGICAL_EXECUTION_BYTES,
        )?,
        finalized: scan_import_native_stats(slices.finalized)?,
        lookahead: scan_import_lookahead_stats(slices.lookahead)?,
    };
    Ok(LogicalImportWirePreflight {
        bytes,
        budget: *budget,
        slices,
        stats,
    })
}
