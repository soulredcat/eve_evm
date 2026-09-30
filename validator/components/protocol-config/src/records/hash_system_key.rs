use alloy_primitives::{B256, keccak256};

use super::{RecordError, SystemNamespace, namespace_bytes::namespace_bytes};
use crate::encoding::encode_list;

pub fn hash_system_key(
    namespace: SystemNamespace,
    logical_key: &[u8],
) -> Result<B256, RecordError> {
    if logical_key.is_empty() || logical_key.len() > 128 {
        return Err(RecordError::InvalidKey);
    }
    Ok(keccak256(encode_list(&[
        alloy_rlp::encode(namespace_bytes(namespace)),
        alloy_rlp::encode(logical_key),
    ])))
}
