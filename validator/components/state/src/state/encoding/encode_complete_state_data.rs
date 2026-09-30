use super::encode_state_data_with_history::encode_state_data_with_history;
use crate::{CompleteState, StateError};

pub(crate) fn encode_complete_state_data(state: &CompleteState) -> Result<Vec<u8>, StateError> {
    encode_state_data_with_history(state, None)
}
