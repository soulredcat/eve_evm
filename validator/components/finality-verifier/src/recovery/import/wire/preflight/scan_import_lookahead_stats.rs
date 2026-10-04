// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{ImportLookaheadWireStats, ImportWireError},
    scan_import_native_stats::scan_import_native_stats,
};
use crate::recovery::{
    bounds::{
        scan_native_data_frame::scan_native_data_frame,
        types::{MAXIMUM_NATIVE_FRAME_BYTES, MAXIMUM_TRANSACTION_BYTES},
    },
    decoding::{take_length_prefixed::take_length_prefixed, take_u32::take_u32},
};

pub(in crate::recovery::import) fn scan_import_lookahead_stats(
    bytes: &[u8],
) -> Result<ImportLookaheadWireStats, ImportWireError> {
    scan_native_data_frame(bytes).map_err(ImportWireError::Recovery)?;
    let mut remaining = bytes;
    let frame = take_length_prefixed(&mut remaining, MAXIMUM_NATIVE_FRAME_BYTES)
        .map_err(ImportWireError::Recovery)?;
    let transaction_count = take_u32(&mut remaining).map_err(ImportWireError::Recovery)?;
    let mut transaction_bytes = 0_usize;
    for _ in 0..transaction_count {
        let transaction = take_length_prefixed(&mut remaining, MAXIMUM_TRANSACTION_BYTES)
            .map_err(ImportWireError::Recovery)?;
        transaction_bytes = transaction_bytes
            .checked_add(transaction.len())
            .ok_or(ImportWireError::BudgetExceeded)?;
    }
    Ok(ImportLookaheadWireStats {
        encoded_bytes: bytes.len(),
        frame: scan_import_native_stats(frame)?,
        transaction_count,
        transaction_bytes,
    })
}
