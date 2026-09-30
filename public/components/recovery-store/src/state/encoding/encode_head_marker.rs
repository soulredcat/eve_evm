use alloy_primitives::B256;

pub(crate) fn encode_head_marker(height: u64, identity: B256) -> Vec<u8> {
    [&height.to_be_bytes(), identity.as_slice()].concat()
}
