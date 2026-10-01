// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{
    OpaqueRecord, OpaqueRecordRepository,
    encoding::{decode_opaque_record, opaque_record_key},
    hashing::hash_opaque_record,
};
use anyhow::{Result, bail};
use std::sync::atomic::Ordering;

/// Read one known retained row; any missing/corrupt/I/O outcome fences this owner.
pub(in crate::records) fn load_opaque_record(
    store: &OpaqueRecordRepository,
    sequence: u64,
) -> Result<OpaqueRecord> {
    let bytes = match store.database.get_pinned(opaque_record_key(sequence)) {
        Ok(Some(bytes)) => bytes,
        Ok(None) => {
            store.fenced.store(true, Ordering::Release);
            bail!("opaque retained record missing; handle fenced");
        }
        Err(error) => {
            store.fenced.store(true, Ordering::Release);
            bail!("opaque record read failed; handle fenced: {error}");
        }
    };
    let record = match decode_opaque_record(&bytes, &store.budget) {
        Ok(record) => record,
        Err(error) => {
            store.fenced.store(true, Ordering::Release);
            bail!("opaque record decode failed; handle fenced: {error}");
        }
    };
    if record.sequence != sequence
        || record.parent.sequence.checked_add(1) != Some(sequence)
        || record.content_hash
            != hash_opaque_record(store.identity, record.parent, sequence, &record.payload)
        || (sequence == store.head.sequence && record.content_hash != store.head.content_hash)
    {
        store.fenced.store(true, Ordering::Release);
        bail!("opaque record integrity mismatch; handle fenced");
    }
    Ok(record)
}
