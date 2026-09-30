use crate::recovery::types::RECORD_PREFIX;

pub fn record_key(height: u64) -> Vec<u8> {
    [RECORD_PREFIX, &height.to_be_bytes()].concat()
}
