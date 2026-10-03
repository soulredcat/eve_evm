// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{StateBudget, preflight_state_journal};

use super::{
    super::{
        ImportWireError, ImportWirePreflight, ImportWireStats,
        framing::slice_import_wire::slice_import_wire,
    },
    scan_import_execution_stats::scan_import_execution_stats,
    scan_import_lookahead_stats::scan_import_lookahead_stats,
    scan_import_native_stats::scan_import_native_stats,
};

/// Borrowed preflight BEFORE decoded allocations; count facts remain bound to the
/// same immutable raw bytes and copied budget. No root/certificate is verified.
pub fn preflight_authenticated_import_wire<'a>(
    bytes: &'a [u8],
    budget: &StateBudget,
) -> Result<ImportWirePreflight<'a>, ImportWireError> {
    let slices = slice_import_wire(bytes, budget)?;
    let stats = ImportWireStats {
        encoded_bytes: bytes.len(),
        journal: preflight_state_journal(slices.journal, budget).map_err(ImportWireError::State)?,
        execution: scan_import_execution_stats(slices.execution, budget)?,
        finalized: scan_import_native_stats(slices.finalized)?,
        lookahead: scan_import_lookahead_stats(slices.lookahead)?,
    };
    Ok(ImportWirePreflight {
        bytes,
        budget: *budget,
        slices,
        stats,
    })
}
