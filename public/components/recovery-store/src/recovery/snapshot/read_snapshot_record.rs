use anyhow::{Result, ensure};

use crate::recovery::encoding::blocks::{
    decode_stored_block::decode_stored_block, record_key::record_key,
};
use crate::recovery::types::{RecordSnapshot, StoredBlockInput};

pub fn read_snapshot_record(
    snapshot: &RecordSnapshot<'_>,
    height: u64,
) -> Result<Option<StoredBlockInput>> {
    let Some(bytes) = snapshot.snapshot.get(record_key(height))? else {
        return Ok(None);
    };
    let record = decode_stored_block(&bytes, snapshot.budget.max_record_bytes)?;
    ensure!(
        record.identity == snapshot.identity && record.height == height,
        "snapshot record identity/height mismatch"
    );
    Ok(Some(record))
}
