use crate::{CompleteState, StateVersion};
use std::sync::Arc;

/// Immutable locally consistent view. No authentication/durability is inferred.
#[derive(Clone, Debug)]
pub struct StateView {
    pub(crate) state: Arc<CompleteState>,
    pub(crate) version: StateVersion,
}
