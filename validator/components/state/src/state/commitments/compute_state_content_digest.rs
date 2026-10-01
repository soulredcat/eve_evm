// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::encoding::encode_complete_state_data;
use crate::{CompleteState, StateBudget, StateError, validate_complete_state};
use alloy_primitives::{B256, keccak256};

pub fn compute_state_content_digest(
    state: &CompleteState,
    budget: &StateBudget,
) -> Result<B256, StateError> {
    validate_complete_state(state, budget)?;
    Ok(keccak256(encode_complete_state_data(state)?))
}
