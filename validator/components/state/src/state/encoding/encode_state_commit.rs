// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{encode_complete_state_data, encode_list, encode_version};
use crate::{StateBudget, StateCommit, StateError, validate_state_commit};

pub fn encode_state_commit(
    commit: &StateCommit,
    budget: &StateBudget,
) -> Result<Vec<u8>, StateError> {
    validate_state_commit(commit, budget)?;
    let parent = match &commit.parent {
        None => encode_list(&[alloy_rlp::encode(0_u8)]),
        Some(version) => encode_list(&[alloy_rlp::encode(1_u8), encode_version(version)]),
    };
    let transactions = commit
        .block
        .transactions
        .iter()
        .map(|bytes| alloy_rlp::encode(bytes.as_ref()))
        .collect::<Vec<_>>();
    let receipts = commit
        .block
        .receipts
        .iter()
        .map(|bytes| alloy_rlp::encode(bytes.as_ref()))
        .collect::<Vec<_>>();
    let encoded = encode_list(&[
        alloy_rlp::encode(b"EVE_STATE_COMMIT_V1".as_slice()),
        parent,
        encode_version(&commit.target),
        encode_complete_state_data(&commit.state)?,
        encode_list(&[
            alloy_rlp::encode(&commit.block.header),
            encode_list(&transactions),
            encode_list(&receipts),
        ]),
    ]);
    if encoded.len() > budget.maximum_commit_bytes {
        return Err(StateError::BudgetExceeded);
    }
    Ok(encoded)
}
