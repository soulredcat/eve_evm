use std::path::Path;

use anyhow::{Result, ensure};
use rocksdb::checkpoint::Checkpoint;

use crate::recovery::records::read_record_cursor;
use crate::recovery::types::{DurableRecordCursor, DurableRecordStore};

/// Create a consistent local physical checkpoint; no source authentication is implied.
pub fn create_record_checkpoint(
    store: &mut DurableRecordStore,
    path: &Path,
) -> Result<DurableRecordCursor> {
    ensure!(!path.exists(), "checkpoint destination already exists");
    let cursor = read_record_cursor(store)?;
    store.database.flush_wal(true)?;
    Checkpoint::new(store.database.as_ref())?.create_checkpoint(path)?;
    Ok(cursor)
}
