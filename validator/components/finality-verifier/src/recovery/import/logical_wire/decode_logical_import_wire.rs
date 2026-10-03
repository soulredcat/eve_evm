// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{super::wire::ImportWireError, LogicalImportWirePreflight, encode_logical_import_wire};
use crate::recovery::{
    decoding::{
        decode_native_data_frame::decode_native_data_frame,
        decode_native_frame::decode_native_frame,
    },
    import::AuthenticatedImportInput,
};
use eve_state::{decode_block_payload, decode_state_journal};

/// Decode only this sealed immutable input under its frozen policy. Full canonical
/// reencoding allocates additional component/output buffers; callers charge them.
/// No imported/replayed capability or parent/finality authority is created here.
pub fn decode_logical_import_wire(
    preflight: &LogicalImportWirePreflight<'_>,
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
    if encode_logical_import_wire(&input, &preflight.budget)? != preflight.bytes {
        return Err(ImportWireError::NonCanonicalEncoding);
    }
    Ok(input)
}
