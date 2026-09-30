use super::StateService;
use crate::state::StateReader;
use std::sync::RwLock;

pub fn create_state_service(reader: StateReader) -> StateService {
    StateService {
        reader,
        cache: RwLock::new(None),
    }
}
