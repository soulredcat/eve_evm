use crate::recovery::types::{IDENTITY_BYTES, MAGIC, StorageIdentity};

/// Storage-only fixed-width bytes; never substitute these for consensus RLP.
pub fn encode_store_identity(identity: &StorageIdentity) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(IDENTITY_BYTES);
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&identity.network.0);
    bytes.extend_from_slice(&identity.genesis_hash);
    bytes.extend_from_slice(&identity.base_height.to_be_bytes());
    bytes.extend_from_slice(&identity.base_block_hash);
    bytes
}
