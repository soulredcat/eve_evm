// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::encoding::encode_state_data_with_history;
use alloy_consensus::Header;
use alloy_primitives::keccak256;
use eve_protocol_config::records::{
    ApplicationCommitmentInput, ExecutionBlockHash, hash_application_commitment,
};

use super::validate_complete_state;
use crate::{
    CompleteState, StateBudget, StateError, StateVersion, compute_evm_root, compute_system_root,
};

pub fn build_state_version(
    state: &CompleteState,
    header: &Header,
    budget: &StateBudget,
) -> Result<StateVersion, StateError> {
    validate_complete_state(state, budget)?;
    let evm_root = compute_evm_root(&state.accounts);
    let system_root = compute_system_root(&state.system)?;
    if evm_root.0 != header.state_root {
        return Err(StateError::CommitMismatch);
    }
    let execution_hash = ExecutionBlockHash(header.hash_slow());
    if state
        .block_hashes
        .keys()
        .any(|height| *height > header.number)
    {
        return Err(StateError::VersionMismatch);
    }
    let content_digest = keccak256(encode_state_data_with_history(
        state,
        Some((header.number, execution_hash)),
    )?);
    let application = if header.number == 0 {
        None
    } else {
        Some(
            hash_application_commitment(ApplicationCommitmentInput {
                genesis: state.identity.genesis,
                protocol_version: state.identity.protocol_version,
                execution_height: header.number,
                evm_root,
                system_root,
                execution_hash,
            })
            .map_err(StateError::SystemRecord)?,
        )
    };
    Ok(StateVersion {
        identity: state.identity.clone(),
        height: header.number,
        timestamp: header.timestamp,
        execution_hash,
        evm_root,
        system_root,
        application,
        content_digest,
    })
}
