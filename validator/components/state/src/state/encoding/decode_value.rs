use crate::StateError;
use alloy_rlp::Decodable;

pub(crate) fn decode_value<T: Decodable>(input: &mut &[u8]) -> Result<T, StateError> {
    T::decode(input).map_err(|_| StateError::MalformedEncoding)
}
