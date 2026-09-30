use crate::StateError;

pub(crate) fn list_length(payload: usize) -> Result<usize, StateError> {
    payload
        .checked_add(alloy_rlp::length_of_length(payload))
        .ok_or(StateError::ArithmeticOverflow)
}
