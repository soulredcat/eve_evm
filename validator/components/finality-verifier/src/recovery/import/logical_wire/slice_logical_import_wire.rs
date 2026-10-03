// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::wire::{ImportWireError, ImportWireSlices},
    types::{
        LOGICAL_IMPORT_DOMAIN, MAXIMUM_LOGICAL_EXECUTION_BYTES, MAXIMUM_LOGICAL_FINALIZED_BYTES,
        MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES, MAXIMUM_LOGICAL_JOURNAL_BYTES,
        MAXIMUM_LOGICAL_LOOKAHEAD_BYTES,
    },
};
use crate::recovery::decoding::take_length_prefixed::take_length_prefixed;
use eve_state::StateBudget;

pub(super) fn slice_logical_import_wire<'a>(
    bytes: &'a [u8],
    budget: &StateBudget,
) -> Result<ImportWireSlices<'a>, ImportWireError> {
    if bytes.len() > MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES {
        return Err(ImportWireError::BudgetExceeded);
    }
    let mut remaining = bytes
        .strip_prefix(LOGICAL_IMPORT_DOMAIN)
        .ok_or(ImportWireError::MalformedEncoding)?;
    let slices = ImportWireSlices {
        journal: take_length_prefixed(
            &mut remaining,
            budget
                .maximum_journal_bytes
                .min(MAXIMUM_LOGICAL_JOURNAL_BYTES),
        )
        .map_err(ImportWireError::Recovery)?,
        execution: take_length_prefixed(
            &mut remaining,
            budget
                .maximum_commit_bytes
                .min(MAXIMUM_LOGICAL_EXECUTION_BYTES),
        )
        .map_err(ImportWireError::Recovery)?,
        finalized: take_length_prefixed(&mut remaining, MAXIMUM_LOGICAL_FINALIZED_BYTES)
            .map_err(ImportWireError::Recovery)?,
        lookahead: take_length_prefixed(&mut remaining, MAXIMUM_LOGICAL_LOOKAHEAD_BYTES)
            .map_err(ImportWireError::Recovery)?,
    };
    if !remaining.is_empty() {
        return Err(ImportWireError::NonCanonicalEncoding);
    }
    Ok(slices)
}
