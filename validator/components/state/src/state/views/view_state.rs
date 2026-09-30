use super::StateView;
use crate::CompleteState;

pub fn view_state(view: &StateView) -> &CompleteState {
    &view.state
}
