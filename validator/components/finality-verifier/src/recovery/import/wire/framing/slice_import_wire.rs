// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::StateBudget;

use super::super::{
    ImportWireError, ImportWireSlices, MAXIMUM_IMPORT_WIRE_BYTES, types::IMPORT_DOMAIN,
};
use crate::recovery::{
    bounds::types::MAXIMUM_NATIVE_FRAME_BYTES, decoding::take_length_prefixed::take_length_prefixed,
};

pub(in crate::recovery::import::wire) fn slice_import_wire<'a>(
    bytes: &'a [u8],
    budget: &StateBudget,
) -> Result<ImportWireSlices<'a>, ImportWireError> {
    if bytes.len() > MAXIMUM_IMPORT_WIRE_BYTES {
        return Err(ImportWireError::BudgetExceeded);
    }
    let mut remaining = bytes
        .strip_prefix(IMPORT_DOMAIN)
        .ok_or(ImportWireError::MalformedEncoding)?;
    let slices = ImportWireSlices {
        journal: take_length_prefixed(
            &mut remaining,
            budget.maximum_journal_bytes.min(MAXIMUM_IMPORT_WIRE_BYTES),
        )
        .map_err(ImportWireError::Recovery)?,
        execution: take_length_prefixed(
            &mut remaining,
            budget.maximum_commit_bytes.min(MAXIMUM_IMPORT_WIRE_BYTES),
        )
        .map_err(ImportWireError::Recovery)?,
        finalized: take_length_prefixed(&mut remaining, MAXIMUM_NATIVE_FRAME_BYTES)
            .map_err(ImportWireError::Recovery)?,
        lookahead: take_length_prefixed(&mut remaining, MAXIMUM_IMPORT_WIRE_BYTES)
            .map_err(ImportWireError::Recovery)?,
    };
    if !remaining.is_empty() {
        return Err(ImportWireError::NonCanonicalEncoding);
    }
    Ok(slices)
}
