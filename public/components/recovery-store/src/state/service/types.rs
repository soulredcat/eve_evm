use crate::state::{ImmutableStateView, StateReader};
use std::sync::{Arc, RwLock};

/// Public query cache owner; the independent repository writer never holds this lock over I/O.
pub struct StateService {
    pub(crate) reader: StateReader,
    pub(crate) cache: RwLock<Option<Arc<ImmutableStateView>>>,
}
