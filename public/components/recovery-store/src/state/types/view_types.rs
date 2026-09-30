use eve_state::StateCommit;

/// Complete read-only local view. External callers cannot mutate cached source fields.
#[derive(Debug)]
pub struct ImmutableStateView {
    pub(crate) commit: StateCommit,
    pub(crate) database_sequence: u64,
}
