use anyhow::{Context, Result, ensure};

use crate::recovery::types::{DurableRecordCursor, StoredBlockInput};

pub fn validate_record_sequence(
    cursor: &DurableRecordCursor,
    records: &[StoredBlockInput],
) -> Result<DurableRecordCursor> {
    let mut next = *cursor;
    for record in records {
        ensure!(
            record.identity == cursor.identity,
            "record network/genesis/bootstrap mismatch"
        );
        ensure!(
            record.parent_height == next.height && record.parent_block_hash == next.block_hash,
            "record parent mismatch"
        );
        ensure!(
            record.height == next.height.checked_add(1).context("height overflow")?,
            "nonconsecutive record height"
        );
        ensure!(
            !record.payload.is_empty(),
            "missing recoverable block payload"
        );
        next.height = record.height;
        next.block_hash = record.block_hash;
    }
    Ok(next)
}
