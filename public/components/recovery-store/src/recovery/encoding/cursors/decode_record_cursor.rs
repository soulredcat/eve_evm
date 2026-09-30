use anyhow::{Result, ensure};

use crate::recovery::encoding::identity::decode_store_identity::decode_store_identity;
use crate::recovery::types::{CURSOR_BYTES, DurableRecordCursor, IDENTITY_BYTES};

pub fn decode_record_cursor(bytes: &[u8]) -> Result<DurableRecordCursor> {
    ensure!(bytes.len() == CURSOR_BYTES, "invalid durable cursor length");
    Ok(DurableRecordCursor {
        identity: decode_store_identity(&bytes[..IDENTITY_BYTES])?,
        height: u64::from_be_bytes(bytes[112..120].try_into()?),
        block_hash: bytes[120..152].try_into()?,
    })
}
