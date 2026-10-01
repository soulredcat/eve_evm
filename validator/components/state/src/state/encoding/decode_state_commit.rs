// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_block_payload::decode_block_payload,
    decode_complete_state_data::decode_complete_state_data, decode_version::decode_version,
    take_bytes, take_list,
};
use crate::{StateBudget, StateCommit, StateError, encode_state_commit, validate_state_commit};

pub fn decode_state_commit(bytes: &[u8], budget: &StateBudget) -> Result<StateCommit, StateError> {
    if bytes.len() > budget.maximum_commit_bytes {
        return Err(StateError::BudgetExceeded);
    }
    let mut remaining = bytes;
    let mut list = take_list(&mut remaining)?;
    if take_bytes(&mut list, 32)?.as_ref() != b"EVE_STATE_COMMIT_V1" {
        return Err(StateError::InvalidSchema);
    }
    let mut parent_list = take_list(&mut list)?;
    let parent = match super::decode_value::<u8>(&mut parent_list)? {
        0 => None,
        1 => Some(decode_version(&mut parent_list)?),
        _ => return Err(StateError::MalformedEncoding),
    };
    if !parent_list.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    let commit = StateCommit {
        parent,
        target: decode_version(&mut list)?,
        state: decode_complete_state_data(&mut list, budget)?,
        block: decode_block_payload(&mut list, budget)?,
    };
    if !remaining.is_empty() || !list.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    validate_state_commit(&commit, budget)?;
    if encode_state_commit(&commit, budget)? != bytes {
        return Err(StateError::NonCanonicalEncoding);
    }
    Ok(commit)
}
