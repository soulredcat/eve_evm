use crate::state::ImmutableStateView;

impl ImmutableStateView {
    pub fn sequence(&self) -> u64 {
        self.database_sequence
    }
}
