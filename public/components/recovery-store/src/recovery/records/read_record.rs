use anyhow::{Result, ensure};

use crate::recovery::encoding::blocks::{
    decode_stored_block::decode_stored_block, record_key::record_key,
};
use crate::recovery::types::{DurableRecordStore, StoredBlockInput};

pub fn read_record(store: &DurableRecordStore, height: u64) -> Result<Option<StoredBlockInput>> {
    let Some(bytes) = store.database.get(record_key(height))? else {
        return Ok(None);
    };
    let record = decode_stored_block(&bytes, store.budget.max_record_bytes)?;
    ensure!(
        record.identity == store.identity && record.height == height,
        "stored record identity/height mismatch"
    );
    Ok(Some(record))
}
