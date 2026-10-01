// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::open_state_namespace;
use crate::state::{StateRepository, StateStorageBudget};
use anyhow::Result;
use eve_state::StateCommit;
use std::path::Path;

/// Open a complete ACTIVE local namespace; no imported bytes authorize finality.
pub fn open_state_repository(
    path: &Path,
    genesis: &StateCommit,
    budget: StateStorageBudget,
) -> Result<StateRepository> {
    open_state_namespace(path, genesis, budget, true)
}
