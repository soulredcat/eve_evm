use crate::StateError;

pub(crate) fn take_list<'a>(input: &mut &'a [u8]) -> Result<&'a [u8], StateError> {
    let header = alloy_rlp::Header::decode(input).map_err(|_| StateError::MalformedEncoding)?;
    if !header.list || header.payload_length > input.len() {
        return Err(StateError::MalformedEncoding);
    }
    let (list, remaining) = input.split_at(header.payload_length);
    *input = remaining;
    Ok(list)
}
