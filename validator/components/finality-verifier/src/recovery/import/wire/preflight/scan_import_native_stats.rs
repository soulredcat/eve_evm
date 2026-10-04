// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{ImportNativeWireStats, ImportWireError},
    count_import_commit_signatures::count_import_commit_signatures,
};
use crate::recovery::{
    bounds::{
        scan_native_frame::scan_native_frame,
        types::{MAXIMUM_BLOCK_ID_BYTES, MAXIMUM_NATIVE_COMMIT_BYTES, MAXIMUM_NATIVE_HEADER_BYTES},
    },
    decoding::take_length_prefixed::take_length_prefixed,
};

pub(in crate::recovery::import) fn scan_import_native_stats(
    bytes: &[u8],
) -> Result<ImportNativeWireStats, ImportWireError> {
    scan_native_frame(bytes).map_err(ImportWireError::Recovery)?;
    let mut remaining = bytes;
    let block_id = take_length_prefixed(&mut remaining, MAXIMUM_BLOCK_ID_BYTES)
        .map_err(ImportWireError::Recovery)?;
    let header = take_length_prefixed(&mut remaining, MAXIMUM_NATIVE_HEADER_BYTES)
        .map_err(ImportWireError::Recovery)?;
    let commit = take_length_prefixed(&mut remaining, MAXIMUM_NATIVE_COMMIT_BYTES)
        .map_err(ImportWireError::Recovery)?;
    Ok(ImportNativeWireStats {
        encoded_bytes: bytes.len(),
        block_id_encoded_bytes: block_id.len(),
        header_encoded_bytes: header.len(),
        commit_encoded_bytes: commit.len(),
        signature_count: count_import_commit_signatures(commit)?,
    })
}
