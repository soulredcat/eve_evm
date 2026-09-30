use crate::state::StateSnapshot;
use eve_state::StateVersion;

impl StateSnapshot<'_> {
    pub fn version(&self) -> &StateVersion {
        &self.version
    }
}
