// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_evm::ExecutionBlockInput;
use eve_state::B256;

use crate::{
    VerifiedDevelopmentHeader,
    recovery::{RecoveryError, types::capability::FixedGenesisPolicy},
};

pub(in crate::recovery) fn derive_authenticated_execution_input(
    certified: &VerifiedDevelopmentHeader,
    policy: &FixedGenesisPolicy,
) -> Result<ExecutionBlockInput, RecoveryError> {
    let header = certified.native().header();
    let time = header
        .time
        .as_ref()
        .ok_or(RecoveryError::InvalidExecutionTime)?;
    if time.seconds < 0 || !(0..1_000_000_000).contains(&time.nanos) {
        return Err(RecoveryError::InvalidExecutionTime);
    }
    let proposer_address: [u8; 20] = header
        .proposer_address
        .as_slice()
        .try_into()
        .map_err(|_| RecoveryError::UnknownProposer)?;
    let proposer = *policy
        .proposer_owners
        .get(&proposer_address)
        .ok_or(RecoveryError::UnknownProposer)?;
    let previous_consensus_hash = if header.height == 1 {
        B256::ZERO
    } else {
        let hash: [u8; 32] = header
            .last_block_id
            .as_ref()
            .ok_or(RecoveryError::WrongParent)?
            .hash
            .as_slice()
            .try_into()
            .map_err(|_| RecoveryError::WrongParent)?;
        B256::from(hash)
    };
    Ok(ExecutionBlockInput {
        timestamp: u64::try_from(time.seconds).map_err(|_| RecoveryError::InvalidExecutionTime)?,
        proposer,
        previous_consensus_hash,
    })
}
