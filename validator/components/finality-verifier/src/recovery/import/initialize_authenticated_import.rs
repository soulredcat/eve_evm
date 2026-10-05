// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_state::{DevelopmentGenesis, StateBudget, initialize_development_state};

use super::{ImportError, ImportedState};
use crate::{
    initialize_development_finality,
    recovery::initialization::build_fixed_genesis_policy::build_fixed_genesis_policy,
};

/// Only the locally selected canonical development genesis initializes import authority.
pub fn initialize_authenticated_import(
    genesis: &DevelopmentGenesis,
    budget: &StateBudget,
) -> Result<ImportedState, ImportError> {
    let commit = initialize_development_state(genesis, budget).map_err(ImportError::State)?;
    let finality = initialize_development_finality(genesis, budget)
        .map_err(|error| ImportError::Recovery(crate::recovery::RecoveryError::Finality(error)))?;
    let policy = build_fixed_genesis_policy(genesis).map_err(ImportError::Recovery)?;
    Ok(ImportedState {
        commit: Arc::new(commit),
        finality,
        policy: Arc::new(policy),
        lookahead: None,
        lookahead_header: None,
        anchor: None,
    })
}
