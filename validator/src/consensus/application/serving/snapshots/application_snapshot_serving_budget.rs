// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::SnapshotServingBudget;
use crate::consensus::application::ConsensusApplication;
pub(in crate::consensus) fn application_snapshot_serving_budget(
    application: &ConsensusApplication,
) -> SnapshotServingBudget {
    application.config.snapshot_serving_budget
}
