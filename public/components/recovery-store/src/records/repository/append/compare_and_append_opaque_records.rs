// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{build_opaque_batch, check_exact_opaque_replay, sync_opaque_batch};
use crate::records::{
    OpaqueRecordAck, OpaqueRecordCursor, OpaqueRecordDisposition, OpaqueRecordRepository,
    opaque_record_cursor, types::schema::RECORD_HEADER_BYTES,
};
use anyhow::{Result, ensure};

/// Atomically sync a new exact-parent batch, or acknowledge a wholly retained exact replay.
pub fn compare_and_append_opaque_records(
    store: &mut OpaqueRecordRepository,
    expected: OpaqueRecordCursor,
    payloads: &[Vec<u8>],
) -> Result<OpaqueRecordAck> {
    let head = opaque_record_cursor(store)?;
    ensure!(
        !payloads.is_empty() && payloads.len() <= store.budget.maximum_batch_records,
        "invalid opaque batch count"
    );
    for payload in payloads {
        ensure!(
            payload
                .len()
                .checked_add(RECORD_HEADER_BYTES)
                .is_some_and(|bytes| bytes <= store.budget.maximum_record_bytes),
            "opaque encoded record byte limit"
        );
    }
    let (batch, appended) = build_opaque_batch(store, expected, payloads)?;
    if expected != head {
        let appended = check_exact_opaque_replay(store, expected, payloads)?;
        return Ok(OpaqueRecordAck {
            appended,
            store_head: head,
            database_sequence: store.database.latest_sequence_number(),
            disposition: OpaqueRecordDisposition::ExactReplay,
        });
    }
    ensure!(
        appended.sequence <= store.budget.maximum_retained_records,
        "opaque retained history capacity exhausted; pruning forbidden"
    );
    sync_opaque_batch(store, batch)?;
    store.head = appended;
    Ok(OpaqueRecordAck {
        appended,
        store_head: appended,
        database_sequence: store.database.latest_sequence_number(),
        disposition: OpaqueRecordDisposition::NewlySynced,
    })
}
