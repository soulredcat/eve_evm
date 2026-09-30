use crate::state::StateSnapshot;

impl StateSnapshot<'_> {
    pub fn sequence(&self) -> u64 {
        self.database_sequence
    }
}
