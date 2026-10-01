// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::decode_opaque_cursor;
use crate::records::{OpaqueRecord, OpaqueRecordBudget, types::schema::RECORD_HEADER_BYTES};
use anyhow::{Result, ensure};

pub(in crate::records) fn decode_opaque_record(
    bytes: &[u8],
    budget: &OpaqueRecordBudget,
) -> Result<OpaqueRecord> {
    ensure!(
        bytes.len() >= RECORD_HEADER_BYTES
            && bytes.len() <= budget.maximum_record_bytes
            && bytes.len() <= budget.maximum_read_bytes,
        "opaque read/record encoded byte limit"
    );
    let payload_length = usize::try_from(u64::from_be_bytes(bytes[80..88].try_into()?))?;
    ensure!(
        payload_length == bytes.len() - RECORD_HEADER_BYTES,
        "opaque payload length mismatch"
    );
    Ok(OpaqueRecord {
        sequence: u64::from_be_bytes(bytes[..8].try_into()?),
        parent: decode_opaque_cursor(&bytes[8..48])?,
        content_hash: bytes[48..80].try_into()?,
        payload: bytes[88..].to_vec(),
    })
}
