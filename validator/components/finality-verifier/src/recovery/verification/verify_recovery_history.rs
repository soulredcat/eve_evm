// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::{
    CompactRecoveryEnvelopeV1, DevelopmentRecoveryState, RecoveryError,
    types::capability::PreparedRecoveryHistory,
};

use super::verify_recovery_frame::verify_recovery_frame;

/// Stage H/H+1 on a clone; failures never advance the accepted parent's history.
pub(in crate::recovery) fn verify_recovery_history(
    parent: &DevelopmentRecoveryState,
    envelope: &CompactRecoveryEnvelopeV1,
) -> Result<PreparedRecoveryHistory, RecoveryError> {
    let mut finality = parent.finality.clone();
    let finalized = match (&parent.lookahead, &parent.lookahead_header) {
        (None, None) if parent.commit.target.height == 0 => verify_recovery_frame(
            &mut finality,
            &envelope.finalized,
            &envelope.execution.transactions,
            &parent.policy,
        )?,
        (Some(previous), Some(header))
            if envelope.finalized == previous.frame
                && envelope.execution.transactions == previous.transactions =>
        {
            header.clone()
        }
        _ => return Err(RecoveryError::WrongLookahead),
    };
    let lookahead = verify_recovery_frame(
        &mut finality,
        &envelope.lookahead.frame,
        &envelope.lookahead.transactions,
        &parent.policy,
    )?;
    Ok(PreparedRecoveryHistory {
        finality,
        finalized,
        lookahead,
    })
}
