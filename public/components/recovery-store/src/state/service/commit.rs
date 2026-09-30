use crate::state::ImmutableStateView;
use eve_state::StateCommit;

impl ImmutableStateView {
    pub fn commit(&self) -> &StateCommit {
        &self.commit
    }
}
