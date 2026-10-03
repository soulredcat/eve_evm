// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::wire::{ImportWireSlices, ImportWireStats};
use eve_state::StateBudget;

pub(super) const LOGICAL_IMPORT_DOMAIN: &[u8] = b"EVE_IMPORT_V2";
pub(super) const MAXIMUM_LOGICAL_JOURNAL_BYTES: usize = 8 * 1_048_576;
pub(super) const MAXIMUM_LOGICAL_EXECUTION_BYTES: usize = 8_404_140;
pub(super) const MAXIMUM_LOGICAL_FINALIZED_BYTES: usize = 16_384;
pub(super) const MAXIMUM_LOGICAL_LOOKAHEAD_BYTES: usize = 4_216_408;

/// Sum of pinned component upper bounds and exact 13-byte/four-u32 framing.
/// These ceilings do not assert all maxima are simultaneously execution-valid.
pub const MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES: usize = 13
    + 16
    + MAXIMUM_LOGICAL_JOURNAL_BYTES
    + MAXIMUM_LOGICAL_EXECUTION_BYTES
    + MAXIMUM_LOGICAL_FINALIZED_BYTES
    + MAXIMUM_LOGICAL_LOOKAHEAD_BYTES;

/// Borrowed raw-input/budget binding, not authenticated import or a resource lease.
/// Detached statistics/slices cannot construct this value or decode other bytes.
#[derive(Debug)]
pub struct LogicalImportWirePreflight<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) budget: StateBudget,
    pub(super) slices: ImportWireSlices<'a>,
    pub(super) stats: ImportWireStats,
}
