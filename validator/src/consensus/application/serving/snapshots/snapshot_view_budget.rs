// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::DeltaServingBudget;
use super::types::SnapshotServingBudget;
pub(super) fn snapshot_view_budget(budget: SnapshotServingBudget) -> DeltaServingBudget {
    DeltaServingBudget {
        maximum_working_bytes: budget.maximum_working_bytes,
        maximum_request_bytes: budget.maximum_request_bytes,
        maximum_chunk_bytes: budget.maximum_chunk_bytes,
        maximum_delta_bytes: budget.maximum_body_bytes,
    }
}
