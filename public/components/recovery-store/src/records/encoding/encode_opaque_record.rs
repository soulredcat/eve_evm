// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::encode_opaque_cursor;
use crate::records::{OpaqueRecord, types::schema::RECORD_HEADER_BYTES};
use anyhow::{Result, ensure};

pub(in crate::records) fn encode_opaque_record(
    record: &OpaqueRecord,
    maximum: usize,
) -> Result<Vec<u8>> {
    let length = record
        .payload
        .len()
        .checked_add(RECORD_HEADER_BYTES)
        .ok_or_else(|| anyhow::anyhow!("opaque record length overflow"))?;
    ensure!(
        length <= maximum,
        "opaque record exceeds encoded byte limit"
    );
    let mut bytes = Vec::with_capacity(length);
    bytes.extend_from_slice(&record.sequence.to_be_bytes());
    bytes.extend_from_slice(&encode_opaque_cursor(record.parent));
    bytes.extend_from_slice(&record.content_hash);
    bytes.extend_from_slice(&u64::try_from(record.payload.len())?.to_be_bytes());
    bytes.extend_from_slice(&record.payload);
    Ok(bytes)
}
