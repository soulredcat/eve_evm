// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::SnapshotServingBudget;
use eve_state::StateBudget;
use eve_storage::checkpoints::messages::CheckpointMessageLimits;
pub(super) fn snapshot_message_limits(
    logical: StateBudget,
    budget: SnapshotServingBudget,
) -> CheckpointMessageLimits {
    CheckpointMessageLimits {
        logical,
        maximum_body_bytes: budget.maximum_body_bytes.min(logical.maximum_commit_bytes),
        maximum_manifest_bytes: budget.maximum_manifest_bytes,
        maximum_chunk_bytes: budget.maximum_chunk_bytes,
    }
}
