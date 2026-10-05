// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::{NativeFrame, RecoveryError, types::capability::FixedGenesisPolicy};

/// Native next-set certificates alone cannot authorize EVM owners or key epochs.
pub(in crate::recovery) fn validate_fixed_validator_frame(
    frame: &NativeFrame,
    policy: &FixedGenesisPolicy,
) -> Result<(), RecoveryError> {
    if frame.header.validators_hash.as_slice() != policy.validator_hash
        || frame.header.next_validators_hash.as_slice() != policy.validator_hash
    {
        return Err(RecoveryError::UnsupportedValidatorTransition);
    }
    let proposer: [u8; 20] = frame
        .header
        .proposer_address
        .as_slice()
        .try_into()
        .map_err(|_| RecoveryError::UnknownProposer)?;
    if !policy.proposer_owners.contains_key(&proposer) {
        return Err(RecoveryError::UnknownProposer);
    }
    Ok(())
}
