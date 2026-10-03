// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::Bytes;

use crate::{
    DevelopmentFinalityVerifier, VerifiedDevelopmentHeader,
    recovery::{NativeFrame, RecoveryError, types::capability::FixedGenesisPolicy},
    verify_next_development_header,
};

use super::validate_fixed_validator_frame::validate_fixed_validator_frame;

pub(in crate::recovery) fn verify_recovery_frame(
    finality: &mut DevelopmentFinalityVerifier,
    frame: &NativeFrame,
    transactions: &[Bytes],
    policy: &FixedGenesisPolicy,
) -> Result<VerifiedDevelopmentHeader, RecoveryError> {
    validate_fixed_validator_frame(frame, policy)?;
    // The native API owns Vec<u8> inputs. Bounds are checked before this temporary copy.
    let native_transactions = transactions
        .iter()
        .map(|tx| tx.to_vec())
        .collect::<Vec<_>>();
    verify_next_development_header(
        finality,
        &frame.block_id,
        &frame.header,
        &frame.commit,
        &policy.validators,
        &native_transactions,
    )
    .map_err(RecoveryError::Finality)
}
