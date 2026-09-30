use std::collections::BTreeMap;

use super::{RecordError, SystemRecord, encode_system_record, hash_system_key};
use crate::encoding::encode_list;

/// A bounded canonical export chunk, ordered by canonical trie-key bytes.
pub fn encode_system_export(records: &[SystemRecord]) -> Result<Vec<u8>, RecordError> {
    if records.len() > 1_024 {
        return Err(RecordError::InvalidRecord);
    }
    let mut sorted = BTreeMap::new();
    for record in records {
        let key = hash_system_key(record.namespace, &record.logical_key)?;
        let value = encode_system_record(record)?;
        if sorted.insert(key, value).is_some() {
            return Err(RecordError::DuplicateKey);
        }
    }
    let entries = sorted
        .into_iter()
        .map(|(key, value)| encode_list(&[alloy_rlp::encode(key), value]))
        .collect::<Vec<_>>();
    Ok(encode_list(&[
        alloy_rlp::encode(b"EVE_SYSTEM_EXPORT_V1".as_slice()),
        alloy_rlp::encode(1_u8),
        encode_list(&entries),
    ]))
}
