// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateView;
use crate::{CompleteState, StateBudget, StateError, StateVersion, validate_state_version};
use std::sync::Arc;

pub fn capture_state_view(
    state: CompleteState,
    version: StateVersion,
    budget: &StateBudget,
) -> Result<StateView, StateError> {
    validate_state_version(&state, &version, budget)?;
    Ok(StateView {
        state: Arc::new(state),
        version,
    })
}
