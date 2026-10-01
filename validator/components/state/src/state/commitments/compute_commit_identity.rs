// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{B256, keccak256};

use crate::{StateBudget, StateCommit, StateError, encode_state_commit};

pub fn compute_commit_identity(
    commit: &StateCommit,
    budget: &StateBudget,
) -> Result<B256, StateError> {
    Ok(keccak256(encode_state_commit(commit, budget)?))
}
