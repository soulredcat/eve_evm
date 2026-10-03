// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(in crate::recovery) use super::history_types::{RecoveryHistoryInput, RecoveryHistoryParent};
use super::verify_recovery_frame::verify_recovery_frame;
use crate::recovery::{RecoveryError, types::capability::PreparedRecoveryHistory};

/// One canonical H/H+1 staging operation shared by replay and authenticated import.
pub(in crate::recovery) fn verify_authenticated_recovery_history(
    parent: &RecoveryHistoryParent<'_>,
    input: &RecoveryHistoryInput<'_>,
) -> Result<PreparedRecoveryHistory, RecoveryError> {
    let mut finality = parent.finality.clone();
    let finalized = match (parent.lookahead, parent.lookahead_header) {
        (None, None) if parent.height == 0 => verify_recovery_frame(
            &mut finality,
            input.finalized,
            input.transactions,
            parent.policy,
        )?,
        (Some(previous), Some(header))
            if input.finalized == &previous.frame
                && input.transactions == previous.transactions.as_slice() =>
        {
            header.clone()
        }
        _ => return Err(RecoveryError::WrongLookahead),
    };
    let lookahead = verify_recovery_frame(
        &mut finality,
        &input.lookahead.frame,
        &input.lookahead.transactions,
        parent.policy,
    )?;
    Ok(PreparedRecoveryHistory {
        finality,
        finalized,
        lookahead,
    })
}
