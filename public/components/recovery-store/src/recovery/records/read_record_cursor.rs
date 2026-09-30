use anyhow::{Context, Result, ensure};

use crate::recovery::encoding::cursors::decode_record_cursor::decode_record_cursor;
use crate::recovery::types::{CURSOR_KEY, DurableRecordCursor, DurableRecordStore};

pub fn read_record_cursor(store: &DurableRecordStore) -> Result<DurableRecordCursor> {
    let bytes = store
        .database
        .get(CURSOR_KEY)?
        .context("missing durable record cursor")?;
    let cursor = decode_record_cursor(&bytes)?;
    ensure!(
        cursor.identity == store.identity,
        "durable cursor identity mismatch"
    );
    Ok(cursor)
}
