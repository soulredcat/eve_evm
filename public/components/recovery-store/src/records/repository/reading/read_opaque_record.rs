// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{load_opaque_record, opaque_record_cursor};
use crate::records::{OpaqueRecord, OpaqueRecordRepository};
use anyhow::Result;

/// Zero is a bootstrap cursor, not a row; absence is authoritative only beyond the complete head.
pub fn read_opaque_record(
    store: &OpaqueRecordRepository,
    sequence: u64,
) -> Result<Option<OpaqueRecord>> {
    let head = opaque_record_cursor(store)?;
    if sequence == 0 || sequence > head.sequence {
        return Ok(None);
    }
    Ok(Some(load_opaque_record(store, sequence)?))
}
