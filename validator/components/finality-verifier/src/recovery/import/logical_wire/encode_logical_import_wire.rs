// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::wire::ImportWireError, measure_logical_import_wire, types::LOGICAL_IMPORT_DOMAIN,
};
use crate::recovery::{
    encoding::{
        append_length_prefixed::append_length_prefixed,
        encode_native_data_frame::encode_native_data_frame,
        encode_native_frame::encode_native_frame,
    },
    import::AuthenticatedImportInput,
};
use eve_state::{StateBudget, encode_block_payload, encode_state_journal};

/// Admit exact total before component Vecs. The caller separately charges input,
/// component encodings, bounded codec scratch and the complete output allocation.
pub fn encode_logical_import_wire(
    input: &AuthenticatedImportInput,
    budget: &StateBudget,
) -> Result<Vec<u8>, ImportWireError> {
    let size = measure_logical_import_wire(input, budget)?;
    let journal = encode_state_journal(&input.journal, budget).map_err(ImportWireError::State)?;
    let execution =
        encode_block_payload(&input.execution, budget).map_err(ImportWireError::State)?;
    let finalized = encode_native_frame(&input.finalized).map_err(ImportWireError::Recovery)?;
    let lookahead =
        encode_native_data_frame(&input.lookahead).map_err(ImportWireError::Recovery)?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(size)
        .map_err(|_| ImportWireError::AllocationFailed)?;
    output.extend_from_slice(LOGICAL_IMPORT_DOMAIN);
    for component in [&journal, &execution, &finalized, &lookahead] {
        append_length_prefixed(&mut output, component).map_err(ImportWireError::Recovery)?;
    }
    if output.len() != size {
        return Err(ImportWireError::NonCanonicalEncoding);
    }
    Ok(output)
}
