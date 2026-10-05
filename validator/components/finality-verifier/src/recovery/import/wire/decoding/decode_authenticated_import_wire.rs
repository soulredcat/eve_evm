// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{decode_block_payload, decode_state_journal};

use super::super::{ImportWireError, ImportWirePreflight, encode_authenticated_import_wire};
use crate::recovery::{
    decoding::{
        decode_native_data_frame::decode_native_data_frame,
        decode_native_frame::decode_native_frame,
    },
    import::AuthenticatedImportInput,
};

/// Decode only the same preflighted bytes under its frozen budget. This function
/// authenticates no parent/root and creates no imported or replayed capability.
pub fn decode_authenticated_import_wire(
    preflight: &ImportWirePreflight<'_>,
) -> Result<AuthenticatedImportInput, ImportWireError> {
    let slices = preflight.slices;
    let input = AuthenticatedImportInput {
        journal: decode_state_journal(slices.journal, &preflight.budget)
            .map_err(ImportWireError::State)?,
        execution: decode_block_payload(slices.execution, &preflight.budget)
            .map_err(ImportWireError::State)?,
        finalized: decode_native_frame(slices.finalized).map_err(ImportWireError::Recovery)?,
        lookahead: decode_native_data_frame(slices.lookahead).map_err(ImportWireError::Recovery)?,
    };
    if encode_authenticated_import_wire(&input, &preflight.budget)? != preflight.bytes {
        return Err(ImportWireError::NonCanonicalEncoding);
    }
    Ok(input)
}
