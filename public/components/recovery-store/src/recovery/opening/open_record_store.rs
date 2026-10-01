// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result, ensure};
use rocksdb::{BlockBasedOptions, Cache, DB, DBCompressionType, Options, WriteBatch, WriteOptions};

use crate::recovery::encoding::cursors::encode_record_cursor::encode_record_cursor;
use crate::recovery::encoding::identity::{
    decode_store_identity::decode_store_identity, encode_store_identity::encode_store_identity,
};
use crate::recovery::records::{read_record, read_record_cursor};
use crate::recovery::types::{
    CURSOR_KEY, DurableRecordCursor, DurableRecordStore, IDENTITY_KEY, StorageBudget,
    StorageIdentity,
};
use crate::recovery::validation::validate_storage_budget::validate_storage_budget;

pub fn open_record_store(
    path: &Path,
    identity: StorageIdentity,
    budget: StorageBudget,
) -> Result<DurableRecordStore> {
    validate_storage_budget(&budget)?;
    let mut options = Options::default();
    options.create_if_missing(true);
    options.set_compression_type(DBCompressionType::None);
    options.set_write_buffer_size(budget.write_buffer_bytes);
    options.set_max_write_buffer_number(budget.write_buffer_count);
    options.set_max_background_jobs(budget.max_background_jobs);
    options.set_max_open_files(budget.max_open_files);
    let cache = Cache::new_lru_cache(budget.block_cache_bytes);
    let mut table = BlockBasedOptions::default();
    table.set_block_cache(&cache);
    options.set_block_based_table_factory(&table);
    let database = DB::open(&options, path).context("open durable record namespace")?;
    if let Some(bytes) = database.get(IDENTITY_KEY)? {
        ensure!(
            decode_store_identity(&bytes)? == identity,
            "existing store identity mismatch"
        );
    } else {
        ensure!(
            database.latest_sequence_number() == 0,
            "nonempty store without identity metadata"
        );
        let mut batch = WriteBatch::default();
        batch.put(IDENTITY_KEY, encode_store_identity(&identity));
        batch.put(
            CURSOR_KEY,
            encode_record_cursor(&DurableRecordCursor {
                identity,
                height: identity.base_height,
                block_hash: identity.base_block_hash,
            }),
        );
        ensure!(
            batch.size_in_bytes() <= budget.max_batch_bytes,
            "bootstrap metadata exceeds byte budget"
        );
        let mut writes = WriteOptions::default();
        writes.set_sync(true);
        writes.disable_wal(false);
        database
            .write_opt(batch, &writes)
            .context("sync storage identity/bootstrap")?;
    }
    let store = DurableRecordStore {
        database: Arc::new(database),
        identity,
        budget,
    };
    let cursor = read_record_cursor(&store)?;
    ensure!(
        cursor.identity == identity && cursor.height >= identity.base_height,
        "invalid durable cursor identity/height"
    );
    if cursor.height > identity.base_height {
        let tail = read_record(&store, cursor.height)?
            .context("durable cursor references missing tail")?;
        ensure!(
            tail.block_hash == cursor.block_hash,
            "durable cursor/tail hash mismatch"
        );
    } else {
        ensure!(
            cursor.block_hash == identity.base_block_hash,
            "bootstrap cursor hash mismatch"
        );
    }
    Ok(store)
}
