use anyhow::{Context, Result, ensure};

use crate::recovery::encoding::identity::decode_store_identity::decode_store_identity;
use crate::recovery::types::{IDENTITY_BYTES, RECORD_HEADER_BYTES, StoredBlockInput};

pub fn decode_stored_block(bytes: &[u8], maximum: usize) -> Result<StoredBlockInput> {
    ensure!(
        bytes.len() >= RECORD_HEADER_BYTES && bytes.len() <= maximum,
        "invalid record size"
    );
    let length = usize::try_from(u64::from_be_bytes(bytes[192..200].try_into()?))?;
    ensure!(
        RECORD_HEADER_BYTES
            .checked_add(length)
            .context("record length overflow")?
            == bytes.len(),
        "noncanonical record length"
    );
    Ok(StoredBlockInput {
        identity: decode_store_identity(&bytes[..IDENTITY_BYTES])?,
        parent_height: u64::from_be_bytes(bytes[112..120].try_into()?),
        height: u64::from_be_bytes(bytes[120..128].try_into()?),
        parent_block_hash: bytes[128..160].try_into()?,
        block_hash: bytes[160..192].try_into()?,
        payload: bytes[RECORD_HEADER_BYTES..].to_vec(),
    })
}
