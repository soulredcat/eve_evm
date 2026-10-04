// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    StateCommitDecodeStats, StateCommitPreflight, scan_commit_accounts::scan_commit_accounts,
    scan_commit_block::scan_commit_block, scan_commit_codes::scan_commit_codes,
    scan_commit_history::scan_commit_history, scan_commit_identity::scan_commit_identity,
    scan_commit_system::scan_commit_system,
};
use crate::{
    BOUNDED_STATE_CODEC_SCRATCH_BYTES, StateBudget, StateError,
    state::encoding::{
        decode_value,
        journal_decoding::{scan_version_allocations, take_borrowed_bytes, take_encoded_list},
        take_list,
    },
};

/// Allocation-free same-byte admission. Canonical roots/state/system/header semantics remain decoder work.
pub fn preflight_state_commit<'a>(
    bytes: &'a [u8],
    budget: &StateBudget,
) -> Result<StateCommitPreflight<'a>, StateError> {
    crate::state::limits::validate_state_budget(budget)?;
    if bytes.len() > budget.maximum_commit_bytes {
        return Err(StateError::BudgetExceeded);
    }
    let mut remaining = bytes;
    let mut commit = take_list(&mut remaining)?;
    if take_borrowed_bytes(&mut commit, 32)? != b"EVE_STATE_COMMIT_V1" {
        return Err(StateError::InvalidSchema);
    }
    let mut stats = StateCommitDecodeStats {
        encoded_bytes: bytes.len(),
        bounded_codec_scratch_bytes: BOUNDED_STATE_CODEC_SCRATCH_BYTES,
        ..Default::default()
    };
    let mut parent = take_list(&mut commit)?;
    match decode_value::<u8>(&mut parent)? {
        0 => {}
        1 => {
            stats.parent_network_bytes =
                scan_version_allocations(take_encoded_list(&mut parent, 4_096)?)?
        }
        _ => return Err(StateError::MalformedEncoding),
    }
    if !parent.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    let target_bytes = take_encoded_list(&mut commit, 4_096)?;
    stats.target_network_bytes = scan_version_allocations(target_bytes)?;
    let state_bytes = take_encoded_list(&mut commit, budget.maximum_state_bytes)?;
    stats.state_encoded_bytes = state_bytes.len();
    let mut state_remaining = state_bytes;
    let mut state = take_list(&mut state_remaining)?;
    stats.state_network_bytes = scan_commit_identity(&mut state)?;
    scan_commit_accounts(&mut state, budget, &mut stats)?;
    scan_commit_codes(&mut state, budget, &mut stats)?;
    scan_commit_system(&mut state, budget, &mut stats)?;
    scan_commit_history(&mut state, budget, &mut stats)?;
    if !state.is_empty() || !state_remaining.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    scan_commit_block(&mut commit, budget, &mut stats)?;
    if !remaining.is_empty() || !commit.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    Ok(StateCommitPreflight {
        bytes,
        target_bytes,
        budget: *budget,
        stats,
    })
}
