// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{
    OpaqueRecordCursor, OpaqueRecordIdentity, hashing::hash_opaque_record,
    types::schema::RECORD_HEADER_BYTES,
};
use anyhow::{Result, ensure};

/// Derive a hypothetical next cursor without storage access or acknowledgement.
/// The caller supplies the actual namespace and encoded-record ceiling; parent membership,
/// admission, batch/retention limits, successful sync and finality remain separate checks.
pub fn prospective_opaque_record_cursor(
    identity: OpaqueRecordIdentity,
    parent: OpaqueRecordCursor,
    payload: &[u8],
    maximum_record_bytes: usize,
) -> Result<OpaqueRecordCursor> {
    let encoded_length = payload
        .len()
        .checked_add(RECORD_HEADER_BYTES)
        .ok_or_else(|| anyhow::anyhow!("opaque record length overflow"))?;
    ensure!(
        encoded_length <= maximum_record_bytes,
        "opaque record exceeds encoded byte limit"
    );
    u64::try_from(payload.len())?;
    let sequence = parent
        .sequence
        .checked_add(1)
        .ok_or_else(|| anyhow::anyhow!("opaque sequence overflow"))?;
    Ok(OpaqueRecordCursor {
        sequence,
        content_hash: hash_opaque_record(identity, parent, sequence, payload),
    })
}
