// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::application::{ConsensusApplication, DeltaServingBudget};

pub(in crate::consensus) fn application_delta_serving_budget(
    application: &ConsensusApplication,
) -> DeltaServingBudget {
    application.config.delta_serving_budget
}
