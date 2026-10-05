// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_state::{DevelopmentGenesis, StateBudget, initialize_development_state};

use super::build_fixed_genesis_policy::build_fixed_genesis_policy;
use crate::{
    initialize_development_finality,
    recovery::{DevelopmentRecoveryState, RecoveryError},
};

/// Only locally selected canonical development genesis initializes replay authority.
pub fn initialize_development_recovery(
    genesis: &DevelopmentGenesis,
    budget: &StateBudget,
) -> Result<DevelopmentRecoveryState, RecoveryError> {
    let commit = initialize_development_state(genesis, budget).map_err(RecoveryError::State)?;
    let finality =
        initialize_development_finality(genesis, budget).map_err(RecoveryError::Finality)?;
    let policy = build_fixed_genesis_policy(genesis)?;
    Ok(DevelopmentRecoveryState {
        commit: Arc::new(commit),
        finality,
        policy: Arc::new(policy),
        lookahead: None,
        lookahead_header: None,
        anchor: None,
    })
}
