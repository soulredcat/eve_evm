// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use rocksdb::{WriteBatch, WriteOptions};

use crate::recovery::encoding::blocks::{
    encode_stored_block::encode_stored_block, record_key::record_key,
};
use crate::recovery::encoding::cursors::encode_record_cursor::encode_record_cursor;
use crate::recovery::records::read_record_cursor;
use crate::recovery::types::{
    CURSOR_KEY, DurableRecordCursor, DurableRecordStore, StoredBlockInput,
};
use crate::recovery::validation::check_replayed_prefix::check_replayed_prefix;
use crate::recovery::validation::validate_record_sequence::validate_record_sequence;

/// Sync a structurally ordered record batch. This does not verify or grant finality.
pub fn commit_record_batch(
    store: &mut DurableRecordStore,
    records: &[StoredBlockInput],
) -> Result<DurableRecordCursor> {
    ensure!(
        !records.is_empty() && records.len() <= store.budget.max_batch_records,
        "invalid batch count"
    );
    let current = read_record_cursor(store)?;
    let replayed = check_replayed_prefix(store, records, &current)?;
    let records = &records[replayed..];
    if records.is_empty() {
        return Ok(current);
    }
    let next = validate_record_sequence(&current, records)?;
    let mut batch = WriteBatch::default();
    for record in records {
        batch.put(
            record_key(record.height),
            encode_stored_block(record, store.budget.max_record_bytes)?,
        );
        ensure!(
            batch.size_in_bytes() <= store.budget.max_batch_bytes,
            "batch exceeds byte budget"
        );
    }
    batch.put(CURSOR_KEY, encode_record_cursor(&next));
    ensure!(
        batch.size_in_bytes() <= store.budget.max_batch_bytes,
        "batch metadata exceeds byte budget"
    );
    let mut writes = WriteOptions::default();
    writes.set_sync(true);
    writes.disable_wal(false);
    store
        .database
        .write_opt(batch, &writes)
        .context("atomic WAL-synced record batch")?;
    Ok(next)
}
