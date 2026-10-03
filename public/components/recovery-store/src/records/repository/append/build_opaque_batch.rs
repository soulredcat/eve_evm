// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{
    OpaqueRecord, OpaqueRecordCursor, OpaqueRecordRepository,
    encoding::{encode_opaque_cursor, encode_opaque_record, opaque_record_key},
    prospective_opaque_record_cursor,
    types::schema::HEAD_KEY,
};
use anyhow::{Result, ensure};
use rocksdb::WriteBatch;

pub(in crate::records) fn build_opaque_batch(
    store: &OpaqueRecordRepository,
    expected: OpaqueRecordCursor,
    payloads: &[Vec<u8>],
) -> Result<(WriteBatch, OpaqueRecordCursor)> {
    let mut batch = WriteBatch::default();
    let mut parent = expected;
    for payload in payloads {
        let next = prospective_opaque_record_cursor(
            store.identity,
            parent,
            payload,
            store.budget.maximum_record_bytes,
        )?;
        let sequence = next.sequence;
        let content_hash = next.content_hash;
        let record = OpaqueRecord {
            sequence,
            parent,
            payload: payload.clone(),
            content_hash,
        };
        batch.put(
            opaque_record_key(sequence),
            encode_opaque_record(&record, store.budget.maximum_record_bytes)?,
        );
        ensure!(
            batch.size_in_bytes() <= store.budget.maximum_batch_bytes,
            "opaque batch exceeds physical byte limit"
        );
        parent = next;
    }
    batch.put(HEAD_KEY, encode_opaque_cursor(parent));
    ensure!(
        batch.size_in_bytes() <= store.budget.maximum_batch_bytes,
        "opaque batch/head metadata exceeds physical byte limit"
    );
    Ok((batch, parent))
}
