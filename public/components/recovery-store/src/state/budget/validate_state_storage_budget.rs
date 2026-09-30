use super::StateStorageBudget;
use crate::recovery::validation::validate_storage_budget::validate_storage_budget;
use anyhow::{Result, ensure};

pub fn validate_state_storage_budget(budget: &StateStorageBudget) -> Result<()> {
    validate_storage_budget(&budget.database)?;
    ensure!(
        budget.logical.maximum_accounts > 0
            && budget.logical.maximum_storage_slots > 0
            && budget.logical.maximum_system_records > 0,
        "empty full-state entry budget"
    );
    ensure!(
        budget.logical.maximum_code_bytes > 0
            && budget.logical.maximum_state_bytes >= budget.logical.maximum_code_bytes,
        "invalid full-state/code byte budget"
    );
    ensure!(
        budget.maximum_commit_bytes > 0
            && budget.maximum_commit_bytes <= budget.database.max_batch_bytes,
        "full-state commit exceeds configured write-batch budget"
    );
    ensure!(
        budget.maximum_snapshot_bytes >= budget.logical.maximum_state_bytes
            && budget.maximum_snapshot_bytes >= budget.maximum_commit_bytes,
        "snapshot budget cannot contain its state/commit references"
    );
    ensure!(
        budget.maximum_snapshots > 0,
        "empty concurrent snapshot budget"
    );
    ensure!(
        budget.maximum_snapshot_files > 0,
        "empty snapshot reference budget"
    );
    Ok(())
}
