use alloy_primitives::{B256, keccak256};

pub fn native_event_topic(signature: &str) -> B256 {
    keccak256(signature.as_bytes())
}
