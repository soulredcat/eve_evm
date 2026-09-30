use crate::recovery::types::StorageBudget;
use eve_state::StateBudget;

/// Local resource bounds, not consensus limits or a strict whole-process memory cap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateStorageBudget {
    pub database: StorageBudget,
    pub logical: StateBudget,
    pub maximum_commit_bytes: usize,
    pub maximum_snapshot_bytes: usize,
    pub maximum_snapshots: usize,
    pub maximum_snapshot_files: usize,
}
