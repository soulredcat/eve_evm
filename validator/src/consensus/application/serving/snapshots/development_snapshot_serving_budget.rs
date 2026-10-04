// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::SnapshotServingBudget;
pub(in crate::consensus) fn development_snapshot_serving_budget() -> SnapshotServingBudget {
    SnapshotServingBudget {
        maximum_working_bytes: 64 * 1_048_576,
        maximum_request_bytes: 1_048_576,
        maximum_body_bytes: 32 * 1_048_576,
        maximum_manifest_bytes: 262_144,
        maximum_chunk_bytes: 4_194_304,
    }
}
