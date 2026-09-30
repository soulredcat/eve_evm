use crate::StateError;
use alloy_primitives::Bytes;

pub(crate) fn take_bytes(input: &mut &[u8], maximum: usize) -> Result<Bytes, StateError> {
    let before = *input;
    let header = alloy_rlp::Header::decode(input).map_err(|_| StateError::MalformedEncoding)?;
    if header.list || header.payload_length > maximum || header.payload_length > input.len() {
        return Err(StateError::MalformedEncoding);
    }
    let (payload, remaining) = input.split_at(header.payload_length);
    let bytes = Bytes::copy_from_slice(payload);
    *input = remaining;
    let consumed = before.len() - input.len();
    if alloy_rlp::encode(bytes.as_ref()) != before[..consumed] {
        return Err(StateError::NonCanonicalEncoding);
    }
    Ok(bytes)
}
