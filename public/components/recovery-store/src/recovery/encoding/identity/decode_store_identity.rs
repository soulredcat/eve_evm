use anyhow::{Result, ensure};

use crate::recovery::types::{IDENTITY_BYTES, MAGIC, StorageIdentity, StorageNetworkId};

pub fn decode_store_identity(bytes: &[u8]) -> Result<StorageIdentity> {
    ensure!(
        bytes.len() == IDENTITY_BYTES,
        "invalid storage identity length"
    );
    ensure!(&bytes[..8] == MAGIC, "unsupported storage schema");
    Ok(StorageIdentity {
        network: StorageNetworkId(bytes[8..40].try_into()?),
        genesis_hash: bytes[40..72].try_into()?,
        base_height: u64::from_be_bytes(bytes[72..80].try_into()?),
        base_block_hash: bytes[80..112].try_into()?,
    })
}
