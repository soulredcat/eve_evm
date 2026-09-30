use crate::{BlockExecutionError, BlockOutcome};
use eve_state::{CompleteState, StateError, StateJournal};

#[derive(Debug)]
pub struct CompleteBlockOutcome {
    pub execution: BlockOutcome,
    pub state: CompleteState,
    pub journal: StateJournal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompleteExecutionError {
    State(StateError),
    Execution(BlockExecutionError),
    CloneReservation { required: usize, reserved: usize },
}
