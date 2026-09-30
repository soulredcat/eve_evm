use super::encode_list;

/// Presence tags distinguish absent from an explicitly present zero integer.
pub(crate) fn encode_optional_height(height: Option<u64>) -> Vec<u8> {
    match height {
        None => encode_list(&[alloy_rlp::encode(0_u8)]),
        Some(value) => encode_list(&[alloy_rlp::encode(1_u8), alloy_rlp::encode(value)]),
    }
}
